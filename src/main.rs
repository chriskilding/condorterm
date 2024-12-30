use clap::Parser;
use data::client::Client;
use data::random::RandomBackend;
use data::udp::UdpBackend;
use iocraft::prelude::*;
use std::net::IpAddr;
mod components;
mod data;
mod utils;
use components::instrument_panel::InstrumentPanel;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    /// Host address [example: 127.0.0.1]
    host: IpAddr,

    /// UDP port
    #[arg(long, default_value_t = 55278)]
    port: u32,
}

fn main() {
    let Program { host: _, port: _ } = Program::parse();

    let client = Client::new(RandomBackend {});

    let mut instrument_panel = element! {
       ContextProvider(value: Context::owned(client)) {
            InstrumentPanel()
        }
    };

    smol::block_on(instrument_panel.fullscreen()).unwrap();
}
