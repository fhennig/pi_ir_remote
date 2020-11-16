use std::time;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pulse {
    Short,
    Long,
    Start,
    Unrecognized,
}

impl Pulse {
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
