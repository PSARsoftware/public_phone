use std::borrow::Borrow;
use std::error::Error;
use std::io;
use std::net::{SocketAddr};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Framed};
use futures::{SinkExt, StreamExt};
use futures::stream::{SplitSink, SplitStream};
use tracing::{debug, info};
use uuid::Uuid;
use crate::codec::PeerCodec;
use crate::command::Command;


/// Connection between 2 peers
pub struct Connection {
    pub id: ConnectionId,
    /// remote peer address
    pub remote_peer_addr: SocketAddr,
    /// user name
    pub user_name: String,
    /// connected user name
    pub remote_user_name: String,
    /// stream to write messages to remote peer
    pub sink: Option<SplitSink<Framed<TcpStream, PeerCodec>, Command>>,
    /// stream to read messages from remote peer
    pub stream: Option<SplitStream<Framed<TcpStream, PeerCodec>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ConnectionId {
    /// unique id for remote connection used by current peer
    pub id: Uuid,
    /// peer_name
    pub peer_name: String,
}

impl Borrow<Uuid> for ConnectionId {
    fn borrow(&self) -> &Uuid {
        &self.id
    }
}

impl Borrow<String> for ConnectionId {
    fn borrow(&self) -> &String {
        &self.peer_name
    }
}

impl ConnectionId {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        info!("new connection id : {id}");
        Self {
            id,
            peer_name: String::new(),
        }
    }
}

impl Connection {

    pub fn new(remote_peer_addr: SocketAddr, user_name: String) -> Self {

        Self {
            id: ConnectionId::new(),
            remote_peer_addr,
            remote_user_name: String::from(""),
            user_name,
            sink: None,
            stream: None
        }
    }

    /// this method is used when current peer initiates connection
    pub async fn connect(&mut self) -> Result<(), io::Error> {
        let stream = TcpStream::connect(self.remote_peer_addr).await?;
        let codec = PeerCodec;
        let (sink, input) = codec.framed(stream).split();
        self.sink = Some(sink);
        self.stream = Some(input);
        // tell our name to connected peer
        self.sink.as_mut().unwrap().send(Command::RequestHandshake(self.remote_user_name.clone()))
            .await
            .expect("could not send peer name");
        Ok(())
    }

    /// this method is used when remote peer initiates connection
    pub async fn accept(&mut self, stream: TcpStream) {
        // TODO find out why input.next().await doesn't let spawn a new tokio task
        //tokio::spawn( async move {
            let codec = PeerCodec;
            let (mut sink, mut input) = codec.framed(stream).split();
            while let Some(Ok(command)) = input.next().await {
                debug!("Command {:?}", command);
                match command {
                    Command::RequestHandshake(name) => {
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