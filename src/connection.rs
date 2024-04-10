use std::error::Error;
use std::net::{SocketAddr};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use uuid::Uuid;


/// Connection between 2 peers
pub struct Connection {
    /// unique id for remote connection used by current peer
    pub id: Uuid,
    /// remote peer address
    pub remote_peer_addr: SocketAddr,
    /// connected user name
    pub remote_user_name: String,
    /// stream to write messages to remote peer
    pub write: OwnedWriteHalf,
    /// stream to read messages from remote peer
    pub read: OwnedReadHalf,
}

impl Connection {

    /// this method is used when current peer initiates connection
    pub async fn connect(remote_peer_addr: SocketAddr) -> Result<Self, Box<dyn Error>> {
        let (read, write) = TcpStream::connect(remote_peer_addr).await?.into_split();
        let id = Uuid::new_v4();
        Ok(
            Self {
                id,
                remote_peer_addr,
                // TODO here we need to add method of transmitting user name after handshake
                remote_user_name: String::from(""),
                write,
                read,
            }
        )
    }

    /// this method is used when remote peer initiates connection
    pub fn from_stream(stream: TcpStream) -> Result<Self, Box<dyn Error>> {
        let remote_peer_addr = stream.peer_addr()?;
        let (read, write) = stream.into_split();
        let id = Uuid::new_v4();
        Ok(
            Self {
                id,
                remote_peer_addr,
                remote_user_name: String::from(""),
                write,
                read,
            }
        )
    }

    async fn perform_handshake() -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
}