use crate::Signal;

/// A signal handler provides a method to process a
/// received signal.
pub trait SignalHandler {
    fn handle_signal(&mut self, signal: &Signal);
}

/// The PrintSignalHandler is a basic handler that
/// prints every signal that is received.
pub struct PrintSignalHandler;

impl PrintSignalHandler {
    pub fn new() -> PrintSignalHandler {
        PrintSignalHandler {}
    }
}

impl SignalHandler for PrintSignalHandler {
    fn handle_signal(&mut self, signal: &Signal) {
        println!("{:?}", signal);
    }
}
