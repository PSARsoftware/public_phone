use tracing::Level;

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

    let
}
