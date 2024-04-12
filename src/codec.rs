use std::error::Error;
use std::io;
use std::io::{ErrorKind, Read};
use byteorder::{BigEndian, ReadBytesExt};
use byteorder::ByteOrder;
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
                Command::SendMessage(msg) => {
                    let _ = write_msg(&msg, buf);
                }
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
            2 => {
                let msg = read_msg(src)?;
                Ok(Some(Command::SendMessage(msg)))
            },
            3 => Ok(Some(Command::SendFile)),
            4 => Ok(Some(Command::StartAudioCall)),
            5 => Ok(Some(Command::StartVideoCall)),
            a @ _ => Ok(Some(Command::Other(a))),
        }
    }
}

fn read_msg(src: &mut BytesMut) -> Result<String, Box<dyn Error>> {
    let size = {
        if src.len() < 2 {
            return Err(Box::new(io::Error::new(ErrorKind::InvalidInput, "invalid msg len")));
        }
        BigEndian::read_u16(src.as_ref()) as usize
    };

    if src.len() >= size + 2 {
        let _ = src.split_to(2);
        let buf = src.split_to(size);
        // TODO find better way
        Ok(String::from_utf8(Vec::from(buf))?)
    } else {
        Err(Box::new(io::Error::new(ErrorKind::InvalidInput, "invalid msg data")))
    }
}

fn write_msg(msg: &str, dst: &mut BytesMut) -> Result<(), io::Error> {
    let msg_ref: &[u8] = msg.as_ref();

    dst.reserve(msg_ref.len() + 2);
    dst.put_u16(msg_ref.len() as u16);
    dst.put(msg_ref);

    Ok(())
}