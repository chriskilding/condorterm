use clap::Parser;
use iocraft::prelude::*;
use std::net::IpAddr;
mod components;
mod data;
use components::instrument_panel::InstrumentPanel;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    /// UDP host address [example: 127.0.0.1]
    host: IpAddr,

    /// UDP port
    #[arg(long, default_value_t = 55278)]
    port: u32,
}

fn main() {
    let Program { host, port } = Program::parse();

    let host_str = format!("{}", host);
    let port_str = format!("{}", port);

    let mut instrument_panel = element!(InstrumentPanel(host: host_str, port: port_str));

    smol::block_on(instrument_panel.fullscreen()).unwrap();
}
