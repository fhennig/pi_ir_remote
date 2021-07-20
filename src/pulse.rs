use std::time;

/// The different types of pulses.  These refer only to the "on" pulse,
/// the "off" pulses are always the same duration.  The main pulse types
/// are "Short" and "Long" (around 500µs and 1500µs), which are used to
/// transmit binary encoded button IDs.
///
/// There are also other pulse lengths, typically orders of magnitude
/// larger, that encode things like the start of a transmition or a button
/// being held down.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pulse {
    Short,
    Long,
    Start,
    Unrecognized,
}

impl Pulse {
    /// Returns the appropriate pulse type for a pulse with a given
    /// duration.
    pub fn from_duration(dur: time::Duration) -> Pulse {
        let dur = dur.as_micros();
        if 0 < dur && dur < 1000 {
            return Pulse::Short;
        } else if 1000 < dur && dur < 2000 {
            return Pulse::Long;
        } else if 2000 < dur && dur < 5000 {
            return Pulse::Start;
        } else {
            return Pulse::Unrecognized;
        }
    }
}

/// A helper function that converts a sequence of pulses to a string,
/// a sequence of characters.
pub fn seq_to_string(seq: &Vec<Pulse>) -> String {
    let mut s = "".to_owned();
    for p in seq {
        if p == &Pulse::Short {
            s += "S";
        } else if p == &Pulse::Long {
            s += "L";
        }
    }
    s.to_string()
}
