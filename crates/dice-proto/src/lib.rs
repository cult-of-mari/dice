use binrw::{BinRead, BinWrite};

pub use self::bool::DiceBool;
pub use self::string::{DiceString, DiceStringError};

mod bool;
mod string;

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct Look {
    pub yaw: f64,
    pub pitch: f64,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct EntityLook {
    pub yaw: i8,
    pub pitch: i8,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct EntityPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct EntityVelocity {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct EntityVelocity2 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i8,
    pub z: i32,
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub enum DicePacket {
    #[brw(magic = 0u8)]
    KeepAlive,

    #[brw(magic = 1u8)]
    Login {
        protocol_version: i32,
        username: DiceString,
        random_seed: i64,
        dimension: u8,
    },

    #[brw(magic = 2u8)]
    Handshake { username: DiceString },

    #[brw(magic = 3u8)]
    Chat { message: DiceString },

    #[brw(magic = 4u8)]
    UpdateTime { time: i64 },

    #[brw(magic = 5u8)]
    PlayerInventory {
        entity_id: i32,
        slot: i16,
        item_id: i16,
        item_damage: i16,
    },

    #[brw(magic = 6u8)]
    SpawnPosition { x: i32, y: i32, z: i32 },

    #[brw(magic = 7u8)]
    Interact {
        player_id: i32,
        target_id: i32,
        left_click: DiceBool,
    },

    #[brw(magic = 8u8)]
    UpdateHealth { health: i16 },

    #[brw(magic = 9u8)]
    Respawn { dimension: i8 },

    #[brw(magic = 10u8)]
    Flying { on_ground: DiceBool },

    #[brw(magic = 11u8)]
    Position {
        position: Position,
        on_ground: DiceBool,
    },

    #[brw(magic = 12u8)]
    Look { look: Look, on_ground: DiceBool },

    #[brw(magic = 13u8)]
    PositionLook {
        position: Position,
        look: Look,
        on_ground: DiceBool,
    },

    #[brw(magic = 14u8)]
    BreakBlock {
        status: DiceBool,
        position: BlockPosition,
        face: u8,
    },

    #[brw(magic = 15u8)]
    PlaceBlock {
        position: BlockPosition,
        direction: u8,
        // TODO
        stack: (),
    },

    #[brw(magic = 16u8)]
    HandSlot { slot: i16 },

    #[brw(magic = 17u8)]
    PlayerSleep {
        entity_id: i32,
        _unused: i8,
        block_position: BlockPosition,
    },

    #[brw(magic = 18u8)]
    EntityAnimation { entity_id: i32, animate: i8 },

    #[brw(magic = 19u8)]
    EntityAction { entity_id: i32, state: i8 },

    #[brw(magic = 20u8)]
    HumanSpawn {
        entity_id: i32,
        username: DiceString,
        position: EntityPosition,
        yaw: i8,
        pitch: i8,
        current_item: i16,
    },

    #[brw(magic = 21u8)]
    ItemSpawn {
        entity_id: i32,
        stack_id: i16,
        stack_size: i8,
        stack_damage: i16,
        position: EntityPosition,
        velocity: EntityVelocity,
    },

    #[brw(magic = 22u8)]
    EntityPickup {
        target_entity_id: i32,
        entity_id: i32,
    },

    #[brw(magic = 23u8)]
    ObjectSpawn {
        entity_id: i32,
        kind: i8,
        position: EntityPosition,
        has_velocity: DiceBool,
        velocity: Option<EntityVelocity2>,
    },

    #[brw(magic = 24u8)]
    MobSpawn {
        entity_id: i32,
        kind: i8,
        position: EntityPosition,
        yaw: i8,
        pitch: i8,
        // metadata
    },

    #[brw(magic = 25u8)]
    PaintingSpawn {
        entity_id: i32,
        title: DiceString,
        position: EntityPosition,
        direction: i32,
    },

    #[brw(magic = 28u8)]
    EntityVelocity {
        entity_id: i32,
        velocity: EntityVelocity2,
    },

    #[brw(magic = 29u8)]
    EntityKill { entity_id: i32 },

    #[brw(magic = 30u8)]
    Entity { entity_id: i32 },

    #[brw(magic = 31u8)]
    EntityMove {
        entity_id: i32,
        velocity: EntityVelocity,
    },

    #[brw(magic = 32u8)]
    EntityLook { entity_id: i32, look: EntityLook },

    #[brw(magic = 33u8)]
    EntityMoveAndLook {
        entity_id: i32,
        velocity: EntityVelocity,
        look: EntityLook,
    },

    #[brw(magic = 34u8)]
    EntityPositionAndLook {
        entity_id: i32,
        position: EntityPosition,
        look: EntityLook,
    },

    #[brw(magic = 38u8)]
    EntityStatus { entity_id: i32, status: i8 },

    #[brw(magic = 39u8)]
    EntityRide {
        entity_id: i32,
        vehicle_entity_id: i32,
    },

    #[brw(magic = 40u8)]
    EntityMetadata {
        entity_id: i32,
        //metadata
    },

    #[brw(magic = 50u8)]
    ChunkState {
        chunk_x: i32,
        chunk_z: i32,
        init: DiceBool,
    },

    #[brw(magic = 51u8)]
    ChunkData {
        x: i32,
        y: i16,
        z: i32,
        x_size: i8,
        y_size: i8,
        z_size: i8,
        data_len: i32,
        #[br(count = data_len as usize)]
        data: Vec<u8>,
    },

    #[brw(magic = 52u8)]
    ChunkBlockSet {
        chunk_x: i32,
        chunk_y: i32,
        blocks_len: i16,
        #[br(count = blocks_len as usize)]
        blocks_position: Vec<i16>,
        #[br(count = blocks_len as usize)]
        blocks_id: Vec<u8>,
        #[br(count = blocks_len as usize)]
        blocks_metadata: Vec<u8>,
    },

    #[brw(magic = 53u8)]
    BlockSet {
        position: BlockPosition,
        block: i8,
        metadata: i8,
    },

    #[brw(magic = 54u8)]
    BlockAction {
        position: BlockPosition,
        data0: i8,
        data1: i8,
    },

    #[brw(magic = 60u8)]
    Explosion {
        x: f64,
        y: f64,
        z: f64,
        size: f32,
        blocks_len: i32,
        #[br(count = blocks_len as usize)]
        blocks: Vec<[i8; 3]>,
    },

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
