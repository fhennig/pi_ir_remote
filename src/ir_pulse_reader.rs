use crate::Pulse;
use crate::Signal;
use crate::SignalHandler;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::collections::VecDeque;
use std::error;
use std::thread;
use std::time;
use stoppable_thread::{spawn, StoppableHandle};

/// Given a pin to read on and a handler for signal, this function
/// polls the pin to receive IR pulses which are decoded into signals
/// and then passed to the handler, when they occurr.  This is running
/// in a dedicated thread.
pub fn read_ir_remote(
    gpio_pin: u32,
    mut signal_handler: Box<dyn SignalHandler + Send + Sync>,
) -> StoppableHandle<()> {
    spawn(move |stopped| {
        let mut reader = IRPulseReader::new(gpio_pin).unwrap();
        while !stopped.get() {
            if let Ok(Some(signal)) = reader.read_pulse() {
                signal_handler.handle_signal(&signal);
            }
        }
        ()
    })
}

/// A struct that allows to detect IR pulses and signals by polling the state
/// of an IR sensor periodically, calculating pulse lengths and decoding pulse
/// sequences into signals.
pub struct IRPulseReader {
    pulse_seq: VecDeque<Pulse>,
    handler: LineHandle,
    prev_val: u8,
    pulse_start: time::SystemTime,
}

impl IRPulseReader {
    pub fn new(gpio_pin: u32) -> Result<IRPulseReader, Box<dyn error::Error>> {
        // Setup
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let handler = chip
            .get_line(gpio_pin)?
            .request(LineRequestFlags::INPUT, 0, "read-input")?;
        Ok(IRPulseReader {
            pulse_seq: vec![].into_iter().collect(),
            handler: handler,
            prev_val: 0,
            pulse_start: time::SystemTime::now(),
        })
    }

    /// This function needs to be called periodically to detect pulses and signals.
    /// An error is returned if the GPIO pin cannot be read.  The function returns
    /// the decoded signal if the current transmission state resulted in a complete
    /// signal.  If no signal was recognized or transmission is in progress, None is
    /// returned.
    pub fn read_pulse(&mut self) -> Result<Option<Signal>, Box<dyn error::Error>> {
        thread::sleep(time::Duration::from_micros(100));
        let mut signal = None;
        let value = self.handler.get_value()?;
        if value != self.prev_val {
            let now = time::SystemTime::now();
            let diff = now.duration_since(self.pulse_start).unwrap();
            let s = format!("{:?} {:?}\n", self.prev_val, diff.as_micros());
            if self.prev_val == 1 {
                println!("{}", s);
                self.pulse_seq.push_back(Pulse::from_duration(diff));
                signal = self.handle_pulse();
            }
            self.prev_val = value;
            self.pulse_start = now;
        }
        Ok(signal)
    }

    /// This function handles the draining of the queue.  The queue of pulses
    /// is drained whenever an unrecognized pulse is received (signal broken)
    /// or the explicit start of a signal is announced with the start pulse.
    /// Once 32 short/long pulses have been gathered, an attempt is made to 
    /// decode the pulse sequence into a signal, which is returned if the
    /// decoding is successful.
    fn handle_pulse(&mut self) -> Option<Signal> {
        let last_pulse = *self.pulse_seq.back().unwrap();
        if last_pulse == Pulse::Start || last_pulse == Pulse::Unrecognized {
            self.pulse_seq.clear();
        }
        if self.pulse_seq.len() == 32 {
            let seq = self.pulse_seq.drain(0..).collect::<Vec<_>>();
            let signal = Signal::from_pulse_seq(&seq);
            return Some(signal);
        }
        return None;
    }
}
