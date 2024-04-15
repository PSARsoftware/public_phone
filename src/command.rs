
#[derive(PartialEq, Debug)]
pub enum Command {
    // request handshake and tell name
    RequestHandshake(String),
    ApproveHandshake,
    GetPeers,
    SendMessage(String),
    SendFile,
    StartAudioCall,
    StartVideoCall,
    Other(u8),
}

impl From<u8> for Command {
    fn from(byte: u8) -> Command {
        match byte {
            0 => Command::RequestHandshake(String::new()),
            1 => Command::ApproveHandshake,
            2 => Command::GetPeers,
            // TODO not idiomatic
            3 => Command::SendMessage(String::new()),
            4 => Command::SendFile,
            5 => Command::StartAudioCall,
            6 => Command::StartVideoCall,
            _ => Command::Other(byte),
        }
    }
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        match self {
            Command::RequestHandshake(_) => 0,
            Command::ApproveHandshake => 1,
            Command::GetPeers => 2,
            Command::SendMessage(_) => 3,
            Command::SendFile => 4,
            Command::StartAudioCall => 5,
            Command::StartVideoCall => 6,
            Command::Other(byte) => byte,
        }
    }
}