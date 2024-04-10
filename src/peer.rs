use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, error, info};
use crate::connection::Connection;

pub struct Peer {
    /// listening address
    socket_addr: SocketAddr,
    connections: HashSet<Connection>,
    state: PeerState,
}

impl Peer {

    pub fn new(socket_addr: SocketAddr) -> Self {
        Self {
            socket_addr,
            connections: Default::default(),
            state: PeerState::Started,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.socket_addr).await?;
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("peer {addr} connected");
                    let mut new_conn = Connection::from_stream(stream)?;
                    let handled = Self::handle_incoming_connection(&mut new_conn).await;
                    if handled.is_err() {
                        let err = handled.err().unwrap();
                        error!("error while handling connection : {err}");
                    }
                }
                Err(e) => {
                    error!("{e}")
                }
            }
        }
    }

    // TODO here we need to send different commands to remote peer
    pub async fn send_data_to_remote_peer<Data: serde::Serialize>(data: &Data)
        -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    async fn handle_incoming_connection(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
        //let read = &conn.read;
        unimplemented!()
    }

}

// TODO here we need to invent a way to simultaneously receive messages and do other things
pub enum PeerState {
    /// Peer just has started
    Started,
    /// ready for handling incoming connections
    Ready,
    /// state when peer is performing calls (unavailable for other calls)
    Occupied,
    Stopped,
}