use bevy::prelude::Event;
use binrw::{io::NoSeek, BinRead, BinWrite};
use std::io::{self, Cursor};
use tokio_util::{
    bytes::{Buf, BufMut, BytesMut},
    codec::{Decoder, Encoder},
};

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct MinecraftString {
    len: i16,
    #[br(count = len as usize)]
    data: Vec<u16>,
}

impl From<String> for MinecraftString {
    fn from(value: String) -> Self {
        let data: Vec<_> = value.encode_utf16().collect();

        Self {
            len: data.len() as i16,
            data,
        }
    }
}

impl From<MinecraftString> for String {
    fn from(value: MinecraftString) -> Self {
        String::from_utf16_lossy(&value.data)
    }
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big, repr = u8)]
pub enum MinecraftBool {
    False,
    True,
}

#[derive(BinRead, BinWrite, Clone, Debug, Event)]
#[brw(big)]
pub enum MinecraftPacket {
    #[brw(magic = 0u8)]
    KeepAlive,

    #[brw(magic = 1u8)]
    Login {
        version: u32,
        username: MinecraftString,
        seed: u32,
        dimension: u8,
    },

    #[brw(magic = 2u8)]
    Handshake { username: MinecraftString },

    #[brw(magic = 3u8)]
    Chat { message: MinecraftString },

    #[brw(magic = 7u8)]
    Interact {
        player_id: u32,
        target_id: u32,
        left_click: MinecraftBool,
    },

    #[brw(magic = 9u8)]
    Respawn { dimension: u8 },

    #[brw(magic = 10u8)]
    Flying { on_ground: MinecraftBool },

    #[brw(magic = 11u8)]
    Position {
        x: f64,
        y: f64,
        stance: f64,
        z: f64,
        on_ground: MinecraftBool,
    },

    #[brw(magic = 12u8)]
    Look {
        yaw: f32,
        pitch: f32,
        on_ground: MinecraftBool,
    },

    #[brw(magic = 13u8)]
    PositionLook {
        x: f64,
        y: f64,
        stance: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: MinecraftBool,
    },

    #[brw(magic = 14u8)]
    BreakBlock {
        status: MinecraftBool,
        x: i32,
        y: u8,
        z: i32,
        face: u8,
    },

    #[brw(magic = 15u8)]
    PlaceBlock {
        x: i32,
        y: u8,
        z: i32,
        direction: u8,
        // TODO
        stack: (),
    },

    #[brw(magic = 16u8)]
    HandSlot { slot: i16 },

    #[brw(magic = 18u8)]
    Animation {
        entity_id: i32,
        animate: MinecraftBool,
    },

    #[brw(magic = 19u8)]
    Action {
        entity_id: i32,
        state: MinecraftBool,
    },

    #[brw(magic = 101u8)]
    WindowClose { window_id: u8 },

    #[brw(magic = 102u8)]
    WindowClick {
        window_id: u8,
        slot: i16,
        right_click: MinecraftBool,
        transaction_id: i32,
        shift_click: MinecraftBool,
        // TODO
        stack: (),
    },

    #[brw(magic = 106u8)]
    WindowTransaction {
        window_id: u8,
        transaction_id: i32,
        accepted: MinecraftBool,
    },

    #[brw(magic = 130u8)]
    UpdateSign {
        x: i32,
        y: i16,
        z: i32,
        lines: [MinecraftString; 4],
    },

    #[brw(magic = 255u8)]
    Disconnect { reason: MinecraftString },
}

pub struct MinecraftCodec;

impl Decoder for MinecraftCodec {
    type Item = MinecraftPacket;
    type Error = io::Error;

    fn decode(&mut self, source: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = Cursor::new(&source[..]);

        if let Ok(packet) = MinecraftPacket::read(&mut cursor) {
            source.advance(cursor.position() as usize);

            return Ok(Some(packet));
        }

        Ok(None)
    }
}

impl Encoder<MinecraftPacket> for MinecraftCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        packet: MinecraftPacket,
        destination: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        packet
            .write(&mut NoSeek::new(destination.writer()))
            .map_err(io::Error::other)?;

        Ok(())
    }
}
