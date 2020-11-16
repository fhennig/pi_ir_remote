use crate::Signal;

pub trait SignalHandler {
    fn handle_signal(&mut self, signal: &Signal);
}

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