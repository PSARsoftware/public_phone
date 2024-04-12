use std::collections::{HashMap};
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener};
//use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};
use crate::command::Command;
use crate::connection::{Connection, ConnectionId};

pub struct Peer {
    /// listening address
    socket_addr: SocketAddr,
    connections: Arc<Mutex<HashMap<ConnectionId, Connection>>>,
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
        // start listening user commands
        let connections = self.connections.clone();
        std::thread::spawn(move || {
            let mut cmd = String::new();
            loop {
                if io::stdin().read_line(&mut cmd).is_err() {
                    error!("could not read user command");
                    continue
                } else {
                    match cmd.as_str() {
                        "message" => {
                            let mut msg = String::new();
                            let _msg = io::stdin().read_line(&mut msg);
                            let mut conn_name = String::new();
                            let _conn = io::stdin().read_line(&mut conn_name);
                            if let Some(mut connection) = connections.lock().unwrap().get_mut(&conn_name) {
                                let _ = Self::send_command_to_remote_peer(&mut connection, Command::SendMessage(msg));
                            } else {
                                warn!("no {conn_name} connection")
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
        // start listening incoming connections
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("peer {addr} connected");
                    let mut conn = Connection::new(addr);
                    // handle incoming connection
                    conn.accept(stream).await;
                }
                Err(e) => {
                    error!("{e}")
                }
            }
        }
    }

    // TODO here we need to send different commands to remote peer
    pub async fn send_command_to_remote_peer(connection: &mut Connection, command: Command)
        -> Result<(), Box<dyn Error>> {
        connection.send_command_to_peer(command).await
    }

    pub async fn connect_to_peer(&mut self, peer_addr: SocketAddr) -> Result<(), io::Error> {
        let mut connection = Connection::new(peer_addr);
        connection.connect().await?;
        // TODO check deadlock possibility
        let mut connections = self.connections.lock().unwrap();
        connections.insert(connection.id.clone(), connection);
        Ok(())
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