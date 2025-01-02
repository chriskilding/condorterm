use clap::Parser;
use data::client::Client;
use data::random::RandomBackend;
use data::udp::UdpBackend;
use iocraft::prelude::*;
use smol::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::Arc;
mod components;
mod data;
mod utils;
use components::instrument_panel::InstrumentPanel;
use smol_macros::main;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    /// Host address [example: 127.0.0.1]
    host: IpAddr,

    /// UDP port
    #[arg(long, default_value_t = 55278)]
    port: u32,
}

main! {
    async fn main() {
        let Program { host, port } = Program::parse();

        let address = SocketAddr::from((host, port as u16));
        if let Ok(socket) = UdpSocket::bind(address).await {
            let backend = RandomBackend {};
//            let backend = UdpBackend::new(socket);
            let client = Client::new(backend);

            let mut instrument_panel = element! {
                ContextProvider(value: Context::owned(Arc::new(client))) {
                    InstrumentPanel()
                }
            };

            let _ = instrument_panel.fullscreen().await;
        }
    }
}
