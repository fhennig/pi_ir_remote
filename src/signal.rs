/// A signal is a button press that has been transmitted from the remote,
/// but might also include things like "held down" to indicate that the
/// last pressed button is being held down.
///
/// On a technical level, a signal is encoded in pulses of different
/// lengths.
use crate::Pulse;

/// The different buttons on the remote, plus an "Unrecognized" signal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Power,
    PlayPause,
    BrightnessDown,
    BrightnessUp,
    White,
    Red,
    Green,
    Blue,
    Orange1,
    Orange2,
    Orange3,
    Yellow,
    GrassGreen,
    Turquise,
    Petrol,
    DarkPetrol,
    Blue2,
    Violet,
    LightViolet,
    Pink,
    Rose1,
    Rose2,
    Azure1,
    Azure2,
    RedUp,
    RedDown,
    GreenUp,
    GreenDown,
    BlueUp,
    BlueDown,
    Quick,
    Slow,
    Diy1,
    Diy2,
    Diy3,
    Diy4,
    Diy5,
    Diy6,
    Auto,
    Flash,
    Jump3,
    Jump7,
    Fade3,
    Fade7,
    Unrecognized,
}

impl Signal {
    /// Attempts to match a pulse sequence to a signal.  All button codes
    /// are 32 pulses long (32 bits).
    #[rustfmt::skip]
    pub fn from_pulse_seq(pulse_seq: &[Pulse]) -> Signal {
        use Pulse::Long as L;
        use Pulse::Short as S;
        match pulse_seq {
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,S,S,S,S,L,S,L,L,L,L,L,L,S,L] => Signal::Power,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,S,S,S,S,L,S,S,L,L,L,L,L,S,L] => Signal::PlayPause,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,L,L,S,L,S,S,L,S,S,S,L,S,L] => Signal::BrightnessDown,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,L,L,S,L,S,L,L,S,S,S,L,S,L] => Signal::BrightnessUp,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,S,S,S,L,S,L,L,S,L,L,L,S,L] => Signal::White,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,S,L,L,S,L,S,L,L,L,S,S,L,S,L] => Signal::Red,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,S,L,L,S,L,S,S,L,L,S,S,L,S,L] => Signal::Green,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,S,S,S,L,S,S,L,S,L,L,L,S,L] => Signal::Blue,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,S,L,S,L,S,L,L,S,L,S,L,S,L] => Signal::Orange1,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,S,S,L,S,L,S,L,L,L,L,S,L,S,L] => Signal::Orange2,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,L,L,S,S,S,L,L,S,S,S,L,L,L] => Signal::Orange3,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,S,L,L,S,S,S,L,L,L,S,S,L,L,L] => Signal::Yellow,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,S,L,S,L,S,S,L,S,L,S,L,S,L] => Signal::GrassGreen,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,S,S,L,S,L,S,S,L,L,L,S,L,S,L] => Signal::Turquise,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,L,L,S,S,S,S,L,S,S,S,L,L,L] => Signal::Petrol,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,S,L,L,S,S,S,S,L,L,S,S,L,L,L] => Signal::DarkPetrol,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,S,L,S,S,L,S,S,L,L,S,L,L,S,L] => Signal::Blue2,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,L,S,S,L,S,S,L,S,S,L,L,S,L] => Signal::Violet,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,L,L,L,L,S,S,S,L,S,S,S,S,L,L,L] => Signal::LightViolet,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,L,S,L,L,S,S,S,L,S,L,S,S,L,L,L] => Signal::Pink,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,S,L,S,S,L,S,L,L,L,S,L,L,S,L] => Signal::Rose1,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,L,S,S,L,S,L,L,S,S,L,L,S,L] => Signal::Rose2,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,L,L,L,L,S,S,S,S,S,S,S,S,L,L,L] => Signal::Azure1,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,L,S,L,L,S,S,S,S,S,L,S,S,L,L,L] => Signal::Azure2,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,S,L,S,S,S,L,L,S,L,S,L,L,L] => Signal::RedUp,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,S,S,L,S,S,S,L,L,L,L,S,L,L,L] => Signal::RedDown,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,S,L,S,S,S,S,L,S,L,S,L,L,L] => Signal::GreenUp,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,S,S,L,S,S,S,S,L,L,L,S,L,L,L] => Signal::GreenDown,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,L,L,S,L,S,S,S,L,S,S,L,S,L,L,L] => Signal::BlueUp,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,L,S,S,L,S,S,S,L,S,L,L,S,L,L,L] => Signal::BlueDown,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,L,L,S,L,S,S,S,S,S,S,L,S,L,L,L] => Signal::Quick,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,L,S,S,L,S,S,S,S,S,L,L,S,L,L,L] => Signal::Slow,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,L,S,S,S,S,L,L,S,S,L,L,L,L] => Signal::Diy1,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,L,S,S,S,S,S,L,S,S,L,L,L,L] => Signal::Diy2,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,L,L,L,S,S,S,S,L,S,S,S,L,L,L,L] => Signal::Diy3,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,S,L,S,S,S,S,L,L,L,S,L,L,L,L] => Signal::Diy4,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,S,L,S,S,S,S,S,L,L,S,L,L,L,L] => Signal::Diy5,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,L,S,L,S,S,S,S,L,S,L,S,L,L,L,L] => Signal::Diy6,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,L,L,L,S,S,S,S,S,S,S,S,L,L,L,L] => Signal::Auto,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,L,S,L,S,S,S,S,S,S,L,S,L,L,L,L] => Signal::Flash,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,S,L,S,S,S,S,S,L,L,S,L,L,L,L,L] => Signal::Jump3,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,S,L,S,S,S,S,S,S,L,S,L,L,L,L,L] => Signal::Jump7,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,S,L,L,S,S,S,S,S,L,S,S,L,L,L,L,L] => Signal::Fade3,
            [S,S,S,S,S,S,S,S,L,L,L,L,L,L,L,L,L,L,L,S,S,S,S,S,S,S,S,L,L,L,L,L] => Signal::Fade7,
            _ => Signal::Unrecognized,
        }
    }
}
