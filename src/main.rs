use std::net::SocketAddr;
use std::process;
use std::str::FromStr;
use std::time::Duration;
use tracing::{error, info, Level};
use crate::peer::Peer;
use clap::Parser;

mod connection;
mod peer;
mod codec;
mod command;
mod error;
mod cli;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_target(true)
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set tracing subscriber");

    let args = Args::parse();
    let port = args.port;
    let socket_addr = format!("127.0.0.1:{}", port);
    let peer_addr = SocketAddr::from_str(&socket_addr);
    //let connect = args.connect;

    let mut peer = Peer::new(peer_addr.expect("couldn't parse socket address"));
    let _ = peer.start().await;
}

/// Command line args
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Port to start peer on
    #[arg(long)]
    port: u32,
    /// The 'connect_to' arg is None if this peer is first in the network
    #[arg(long)]
    connect: Option<String>,
}
