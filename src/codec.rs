use std::error::Error;
use std::io::Read;
use tokio_util::codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};
use tracing::debug;
use crate::command::Command;

pub struct PeerCodec;

impl Encoder<Command> for PeerCodec {
    type Error = Box<dyn Error>;

    fn encode(&mut self, item: Command, buf: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(
            match item {
                // TODO make keys exchange
                Command::RequestHandshake => {
                    debug!("trying to perform handshake");
                    buf.reserve(6);
                    buf.put(&b"handshake"[..]);
                }
                Command::ApproveHandshake => {
                    debug!("handshake approved");
                    buf.reserve(8);
                    buf.put(&b"approved"[..]);
                }
                Command::GetPeers => {}
                Command::SendMessage => {}
                Command::SendFile => {}
                Command::StartAudioCall => {}
                Command::StartVideoCall => {}
                Command::Other(_) => {}
        })
    }
}

impl Decoder for PeerCodec {
    type Item = Command;
    type Error = Box<dyn Error>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cmd_buf = [0u8, 1];
        let _ = src.reader().read(&mut cmd_buf);
        match cmd_buf[0] {
            0 => {
                debug!("command 'handshake' received");
                Ok(Some(Command::RequestHandshake))
             },
            1 => Ok(Some(Command::GetPeers)),
            2 => Ok(Some(Command::SendMessage)),
            3 => Ok(Some(Command::SendFile)),
            4 => Ok(Some(Command::StartAudioCall)),
            5 => Ok(Some(Command::StartVideoCall)),
            a @ _ => Ok(Some(Command::Other(a))),
        }
    }
}