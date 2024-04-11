use std::collections::{HashMap};
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, error, info};
use uuid::{Uuid};
use crate::command::Command;
use crate::connection::Connection;

pub struct Peer {
    /// listening address
    socket_addr: SocketAddr,
    connections: HashMap<Uuid, Connection>,
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
        info!("peer 🙏 started");
        let listener = TcpListener::bind(self.socket_addr).await?;
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    //tokio::spawn(async move {
                        debug!("peer {addr} connected");
                        let mut conn = Connection::new(addr);
                        // handle incoming connection
                        conn.accept(stream).await;
                    //});
                }
                Err(e) => {
                    error!("{e}")
                }
            }
        }
    }

    // TODO here we need to send different commands to remote peer
    pub async fn send_command_to_remote_peer(&mut self, command: Command, conn_id: Uuid)
        -> Result<(), Box<dyn Error>> {
        let connection = self.connections.get_mut(&conn_id)
            .ok_or(Box::new(io::Error::new(ErrorKind::InvalidData, "no such connection")))?;
        connection.send_command_to_peer(command).await
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