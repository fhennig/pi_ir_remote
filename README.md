# IR Remote Receiver

<p align="center">
    [crates.io](https://crates.io/crates/pi_ir_remote) - [docs.rs](https://docs.rs/pi_ir_remote/0.1.0/pi_ir_remote/) - [GitHub](https://github.com/fhennig/pi_ir_remote)

    <img alt="44-button IR remote" src="https://github.com/fhennig/pi_ir_remote/blob/master/pi_remote.png?raw=true"/>
</p>

This crate provides a signal decoder for the commonly found 44-button
infra red remote used commonly with LED strips.  When an IR sensor is
connected to a GPIO pin, this crate can decode the IR signals into
which button was pressed.

## Usage

This will read signals on GPIO 4 and print them:

    use pi_ir_remote::read_ir_remote;
    use pi_ir_remote::PrintSignalHandler;

    fn main() {
        let handler = PrintSignalHandler::new();
        read_ir_remote(4, Box::new(handler));
    }

You can make your own signal handler by implementing the SignalHandler
trait:

    pub trait SignalHandler {
        fn handle_signal(&mut self, signal: &Signal);
    }

## How it works

The IR remote sends pulse length encoded signals.  Among other special
pulses, there are a short and long pulse, corresponding to 0 and 1,
allowing the transmission of binary data.

Every button is associated to a 32 bit word.  This library includes
the mapping of binary codes to buttons in the Signal enum.
