use bevy::prelude::*;

pub const PROTOCOL_ID: u64 = 0;
pub const TICKS_PER_SECOND: u64 = 20;
pub const CHUNK_SIZE: i32 = 16;
pub const MAX_INVENTORY_SLOTS: u32 = 4 * 9;
pub const HALF_BLOCK: Vec3 = Vec3 {
    x: 0.5,
    y: 0.5,
    z: 0.5,
};

// increase render distance if we build the project in release mode
pub const DEFAULT_RENDER_DISTANCE_CHUNKS: u32 = if cfg!(debug_assertions) { 2 } else { 4 };
