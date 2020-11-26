/// A signal is a button press that has been transmitted from the remote,
/// but might also include things like "held down" to indicate that the 
/// last pressed button is being held down.
/// 
/// On a technical level, a signal is encoded in pulses of different
/// lengths.
use crate::Pulse;
use crate::pulse::seq_to_string;

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
    pub fn from_pulse_seq(pulse_seq: &Vec<Pulse>) -> Signal {
        let seq_as_str = seq_to_string(pulse_seq);
        match seq_as_str.as_str() {
            "SSSSSSSSLLLLLLLLSSSSSSLSLLLLLLSL" => Signal::Power,
            "SSSSSSSSLLLLLLLLLSSSSSLSSLLLLLSL" => Signal::PlayPause,
            "SSSSSSSSLLLLLLLLLSLLLSLSSLSSSLSL" => Signal::BrightnessDown,
            "SSSSSSSSLLLLLLLLSSLLLSLSLLSSSLSL" => Signal::BrightnessUp,
            "SSSSSSSSLLLLLLLLSSLSSSLSLLSLLLSL" => Signal::White,
            "SSSSSSSSLLLLLLLLSSSLLSLSLLLSSLSL" => Signal::Red,
            "SSSSSSSSLLLLLLLLLSSLLSLSSLLSSLSL" => Signal::Green,
            "SSSSSSSSLLLLLLLLLSLSSSLSSLSLLLSL" => Signal::Blue,
            "SSSSSSSSLLLLLLLLSSLSLSLSLLSLSLSL" => Signal::Orange1,
            "SSSSSSSSLLLLLLLLSSSSLSLSLLLLSLSL" => Signal::Orange2,
            "SSSSSSSSLLLLLLLLSSLLLSSSLLSSSLLL" => Signal::Orange3,
            "SSSSSSSSLLLLLLLLSSSLLSSSLLLSSLLL" => Signal::Yellow,
            "SSSSSSSSLLLLLLLLLSLSLSLSSLSLSLSL" => Signal::GrassGreen,
            "SSSSSSSSLLLLLLLLLSSSLSLSSLLLSLSL" => Signal::Turquise,
            "SSSSSSSSLLLLLLLLLSLLLSSSSLSSSLLL" => Signal::Petrol,
            "SSSSSSSSLLLLLLLLLSSLLSSSSLLSSLLL" => Signal::DarkPetrol,
            "SSSSSSSSLLLLLLLLLSSLSSLSSLLSLLSL" => Signal::Blue2,
            "SSSSSSSSLLLLLLLLLSLLSSLSSLSSLLSL" => Signal::Violet,
            "SSSSSSSSLLLLLLLLSLLLLSSSLSSSSLLL" => Signal::LightViolet,
            "SSSSSSSSLLLLLLLLSLSLLSSSLSLSSLLL" => Signal::Pink,
            "SSSSSSSSLLLLLLLLSSSLSSLSLLLSLLSL" => Signal::Rose1,
            "SSSSSSSSLLLLLLLLSSLLSSLSLLSSLLSL" => Signal::Rose2,
            "SSSSSSSSLLLLLLLLLLLLLSSSSSSSSLLL" => Signal::Azure1,
            "SSSSSSSSLLLLLLLLLLSLLSSSSSLSSLLL" => Signal::Azure2,
            "SSSSSSSSLLLLLLLLSSLSLSSSLLSLSLLL" => Signal::RedUp,
            "SSSSSSSSLLLLLLLLSSSSLSSSLLLLSLLL" => Signal::RedDown,
            "SSSSSSSSLLLLLLLLLSLSLSSSSLSLSLLL" => Signal::GreenUp,
            "SSSSSSSSLLLLLLLLLSSSLSSSSLLLSLLL" => Signal::GreenDown,
            "SSSSSSSSLLLLLLLLSLLSLSSSLSSLSLLL" => Signal::BlueUp,
            "SSSSSSSSLLLLLLLLSLSSLSSSLSLLSLLL" => Signal::BlueDown,
            "SSSSSSSSLLLLLLLLLLLSLSSSSSSLSLLL" => Signal::Quick,
            "SSSSSSSSLLLLLLLLLLSSLSSSSSLLSLLL" => Signal::Slow,
            "SSSSSSSSLLLLLLLLSSLLSSSSLLSSLLLL" => Signal::Diy1,
            "SSSSSSSSLLLLLLLLLSLLSSSSSLSSLLLL" => Signal::Diy2,
            "SSSSSSSSLLLLLLLLSLLLSSSSLSSSLLLL" => Signal::Diy3,
            "SSSSSSSSLLLLLLLLSSSLSSSSLLLSLLLL" => Signal::Diy4,
            "SSSSSSSSLLLLLLLLLSSLSSSSSLLSLLLL" => Signal::Diy5,
            "SSSSSSSSLLLLLLLLSLSLSSSSLSLSLLLL" => Signal::Diy6,
            "SSSSSSSSLLLLLLLLLLLLSSSSSSSSLLLL" => Signal::Auto,
            "SSSSSSSSLLLLLLLLLLSLSSSSSSLSLLLL" => Signal::Flash,
            "SSSSSSSSLLLLLLLLSSLSSSSSLLSLLLLL" => Signal::Jump3,
            "SSSSSSSSLLLLLLLLLSLSSSSSSLSLLLLL" => Signal::Jump7,
            "SSSSSSSSLLLLLLLLSLLSSSSSLSSLLLLL" => Signal::Fade3,
            "SSSSSSSSLLLLLLLLLLLSSSSSSSSLLLLL" => Signal::Fade7,
            _ => Signal::Unrecognized,
        }
    }
}
