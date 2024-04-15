use std::collections::{HashMap};
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener};
//use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};
use crate::command::Command;
use crate::connection::{Connection, ConnectionId};

pub struct Peer {
    /// listening address
    socket_addr: SocketAddr,
    name: String,
    connections: Arc<Mutex<HashMap<ConnectionId, Connection>>>,
    state: PeerState,
}

impl Peer {

    pub fn new(socket_addr: SocketAddr, name: String) -> Self {
        Self {
            socket_addr,
            name,
            connections: Default::default(),
            state: PeerState::Started,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        info!("peer 🙏 started");
        let listener = TcpListener::bind(self.socket_addr).await?;
        // start listening user commands
        let connections = self.connections.clone();
        let user_name = self.name.clone();
        std::thread::spawn(move || {
            Self::process_user_input(connections, &user_name.clone());
        });
        // start listening incoming connections
        loop {
            let user_name = self.name.clone();
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("peer {addr} connected");
                    let mut conn = Connection::new(addr, user_name);
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
        let mut connection = Connection::new(peer_addr, self.name.clone());
        connection.connect().await?;
        // TODO check deadlock possibility
        let mut connections = self.connections.lock().unwrap();
        connections.insert(connection.id.clone(), connection);
        Ok(())
    }

    /// temporary solution, unless there is no ui
    fn process_user_input(
        connections: Arc<Mutex<HashMap<ConnectionId, Connection>>>,
        user_name: &str) {
        loop {
            let mut cmd = String::new();
            println!("enter command: \
            \n 1 - send message to peer;\
            \n 2 - get available connections;\
            \n 3 - connect to peer");

            if io::stdin().read_line(&mut cmd).is_err() {
                error!("could not read user command");
                continue
            } else {
                match cmd.as_str().trim() {
                    "1" => {
                        let mut conn_name = String::new();
                        let mut connections = connections.lock().unwrap();
                        // TODO fix these clones
                        let conns = connections.iter()
                            .map(|c| c.0.peer_name.clone())
                            .fold(String::new(), |acc, c| acc.clone() + " " + &c);
                        println!("\n available connections: {}", conns);

                        if !conns.is_empty() {
                            println!("enter connection name:");
                            let _conn = io::stdin().read_line(&mut conn_name);
                            if let Some(mut connection) = connections.get_mut(&conn_name) {
                                println!("enter message:");
                                let mut msg = String::new();
                                let _ = io::stdin().read_line(&mut msg);
                                let _ = Self::send_command_to_remote_peer(
                                    &mut connection,
                                    Command::SendMessage(msg));
                            } else {
                                warn!("no {conn_name} connection")
                            }
                        }
                    }
                    "2" => {
                        let connections = connections.lock().unwrap();
                        let conns = connections.iter()
                            //.map(|c| c.0.name.clone())
                            .map(|c| c.0.id.clone())
                            .fold(String::new(), |acc, c| acc.clone() + " " + &c.to_string());
                        println!("\n available connections: {}", conns);
                    }
                    "3" => {
                        let mut peer_addr = String::new();
                        println!("enter peer socket address");
                        let _ = io::stdin().read_line(&mut peer_addr);
                        let peer_addr = SocketAddr::from_str(peer_addr.trim())
                            .expect("couldn't parse peer socket address");
                        let connections = connections.clone();

                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let connected = rt.block_on(async move {
                            let mut connection = Connection::new(peer_addr, user_name.to_string());
                            connection.connect().await?;
                            info!("peer connected to {peer_addr}");
                            // TODO check deadlock possibility
                            //debug!("getting lock on connections...");
                            let mut connections = connections.lock().unwrap();
                            connections.insert(connection.id.clone(), connection);
                            //debug!("connections lock has been released");
                            Ok::<(), Box<dyn Error + Send + Sync>>(())
                        });
                        debug!("peer {peer_addr} connected: {connected:?}");
                        rt.shutdown_background();
                    }
                    _ => {
                        println!("wrong command")
                    }
                }
            }
        }
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