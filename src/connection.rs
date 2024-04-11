use std::error::Error;
use std::io;
use std::net::{SocketAddr};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Framed};
use futures::{SinkExt, StreamExt};
use futures::stream::{SplitSink, SplitStream};
use tracing::debug;
use uuid::Uuid;
use crate::codec::PeerCodec;
use crate::command::Command;


/// Connection between 2 peers
pub struct Connection {
    /// unique id for remote connection used by current peer
    pub id: Uuid,
    /// remote peer address
    pub remote_peer_addr: SocketAddr,
    /// connected user name
    pub remote_user_name: String,
    /// stream to write messages to remote peer
    pub sink: Option<SplitSink<Framed<TcpStream, PeerCodec>, Command>>,
    /// stream to read messages from remote peer
    pub stream: Option<SplitStream<Framed<TcpStream, PeerCodec>>>,
}

impl Connection {

    pub fn new(remote_peer_addr: SocketAddr) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            remote_peer_addr,
            remote_user_name: String::from(""),
            sink: None,
            stream: None
        }
    }

    /// this method is used when current peer initiates connection
    pub async fn connect(&mut self, remote_peer_addr: SocketAddr) -> Result<(), io::Error> {
        let stream = TcpStream::connect(remote_peer_addr).await?;
        let codec = PeerCodec;
        let (sink, input) = codec.framed(stream).split();
        self.sink = Some(sink);
        self.stream = Some(input);
        Ok(())
    }

    /// this method is used when remote peer initiates connection
    pub async fn accept(&mut self, stream: TcpStream) {
        //tokio::spawn( async move {
            let codec = PeerCodec;
            let (mut sink, mut input) = codec.framed(stream).split();
            while let Some(Ok(command)) = input.next().await {
                debug!("Command {:?}", command);
                match command {
                    Command::RequestHandshake => {
                        debug!("Command handshake received");
                        if let Err(error) = sink.send(Command::ApproveHandshake).await {
                            debug!("An error occurred {}", error);
                        }
                    }
                    _ => {
                        debug!("unimplemented!")
                    }
                }
            }
        //});
    }

    pub async fn send_command_to_peer(&mut self, command: Command) -> Result<(), Box<dyn Error>> {
        self.sink.as_mut().unwrap().send(command).await
    }

    async fn perform_handshake() -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
}