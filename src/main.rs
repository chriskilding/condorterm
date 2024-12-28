use iocraft::prelude::*;
use clap::Parser;
mod components;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    /// Condor host [example: '127.0.0.1']
    host: String,

    /// Condor UDP port
    #[arg(short, long, default_value_t = 55278)]
    port: i32
}



fn main() {
    let Program { host, port } = Program::parse();

    let mut instrument_panel = element!(components::instrument_panel::InstrumentPanel(host, port: format!("{}", port)));

    smol::block_on(instrument_panel.fullscreen()).unwrap();
}
