use binrw::{BinRead, BinWrite};
use derive_more::{Display, Error};
use std::str::FromStr;

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(repr = u8)]
pub enum DiceBool {
    False,
    True,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct DiceString {
    #[br(assert(len >= 0, "len cannot be negative"))]
    len: i16,
    #[br(count = len as usize)]
    data: Vec<u16>,
}

#[derive(Debug, Display, Error)]
#[display("invalid length")]
pub struct DiceStringError;

impl DiceString {
    pub const MAX_LEN: usize = i16::MAX as usize;

    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(&self.data)
    }
}

impl FromStr for DiceString {
    type Err = DiceStringError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.len() < Self::MAX_LEN {
            let data: Vec<_> = string.encode_utf16().collect();

            assert!(data.len() <= string.len());

            Ok(Self {
                len: data.len() as i16,
                data,
            })
        } else {
            Err(DiceStringError)
        }
    }
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub enum DicePacket {
    #[brw(magic = 0u8)]
    KeepAlive,

    #[brw(magic = 1u8)]
    Login {
        version: u32,
        username: DiceString,
        seed: u32,
        dimension: u8,
    },

    #[brw(magic = 2u8)]
    Handshake { username: DiceString },

    #[brw(magic = 3u8)]
    Chat { message: DiceString },

    #[brw(magic = 7u8)]
    Interact {
        player_id: u32,
        target_id: u32,
        left_click: DiceBool,
    },

    #[brw(magic = 9u8)]
    Respawn { dimension: u8 },

    #[brw(magic = 10u8)]
    Flying { on_ground: DiceBool },

    #[brw(magic = 11u8)]
    Position {
        x: f64,
        y: f64,
        stance: f64,
        z: f64,
        on_ground: DiceBool,
    },

    #[brw(magic = 12u8)]
    Look {
        yaw: f32,
        pitch: f32,
        on_ground: DiceBool,
    },

    #[brw(magic = 13u8)]
    PositionLook {
        x: f64,
        y: f64,
        stance: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: DiceBool,
    },

    #[brw(magic = 14u8)]
    BreakBlock {
        status: DiceBool,
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
    Animation { entity_id: i32, animate: DiceBool },

    #[brw(magic = 19u8)]
    Action { entity_id: i32, state: DiceBool },

    #[brw(magic = 101u8)]
    WindowClose { window_id: u8 },

    #[brw(magic = 102u8)]
    WindowClick {
        window_id: u8,
        slot: i16,
        right_click: DiceBool,
        transaction_id: i32,
        shift_click: DiceBool,
        // TODO
        stack: (),
    },

    #[brw(magic = 106u8)]
    WindowTransaction {
        window_id: u8,
        transaction_id: i32,
        accepted: DiceBool,
    },

    #[brw(magic = 130u8)]
    UpdateSign {
        x: i32,
        y: i16,
        z: i32,
        lines: [DiceString; 4],
    },

    #[brw(magic = 255u8)]
    Disconnect { reason: DiceString },
}
