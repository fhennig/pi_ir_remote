use pi_ir_remote::read_ir_remote;
use pi_ir_remote::PrintSignalHandler;

fn main() {
    let handler = PrintSignalHandler::new();
    read_ir_remote(4, Box::new(handler));
}
