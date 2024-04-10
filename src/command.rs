
#[derive(PartialEq, Debug)]
pub enum Command {
    Handshake,
    GetPeers,
    SendMessage,
    SendFile,
    StartAudioCall,
    StartVideoCall,
    Other(u8),
}

impl From<u8> for Command {
    fn from(byte: u8) -> Command {
        match byte {
            0 => Command::Handshake,
            1 => Command::GetPeers,
            2 => Command::SendMessage,
            3 => Command::SendFile,
            4 => Command::StartAudioCall,
            5 => Command::StartVideoCall,
            _ => Command::Other(byte),
        }
    }
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        match self {
            Command::Handshake => 0,
            Command::GetPeers => 1,
            Command::SendMessage => 2,
            Command::SendFile => 3,
            Command::StartAudioCall => 4,
            Command::StartVideoCall => 5,
            Command::Other(byte) => byte,
        }
    }
}