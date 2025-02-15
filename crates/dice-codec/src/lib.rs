use binrw::{io::NoSeek, BinRead, BinWrite};
use dice_proto::DicePacket;
use std::io::{self, Cursor};
use tokio_util::{
    bytes::{Buf, BufMut, BytesMut},
    codec::{Decoder, Encoder},
};

pub struct DiceCodec;

impl Decoder for DiceCodec {
    type Item = DicePacket;
    type Error = io::Error;

    fn decode(&mut self, source: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = Cursor::new(&source[..]);

        if let Ok(packet) = DicePacket::read(&mut cursor) {
            source.advance(cursor.position() as usize);

            return Ok(Some(packet));
        }

        Ok(None)
    }
}

impl Encoder<DicePacket> for DiceCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        packet: DicePacket,
        destination: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        packet
            .write(&mut NoSeek::new(destination.writer()))
            .map_err(io::Error::other)?;

        Ok(())
    }
}
