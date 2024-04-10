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
                Command::Handshake => {
                    debug!("trying to perform handshake");
                    buf.reserve(6);
                    buf.put(&b"secret"[..]);
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
    type Item = ();
    type Error = Box<dyn Error>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let
        match src.reader().read() {  }
    }
}