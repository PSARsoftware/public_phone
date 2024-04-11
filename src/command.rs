
#[derive(PartialEq, Debug)]
pub enum Command {
    RequestHandshake,
    ApproveHandshake,
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
            0 => Command::RequestHandshake,
            2 => Command::ApproveHandshake,
            3 => Command::GetPeers,
            4 => Command::SendMessage,
            5 => Command::SendFile,
            6 => Command::StartAudioCall,
            7 => Command::StartVideoCall,
            _ => Command::Other(byte),
        }
    }
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        match self {
            Command::RequestHandshake => 0,
            Command::ApproveHandshake => 1,
            Command::GetPeers => 2,
            Command::SendMessage => 3,
            Command::SendFile => 4,
            Command::StartAudioCall => 5,
            Command::StartVideoCall => 6,
            Command::Other(byte) => byte,
        }
    }
}