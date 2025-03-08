use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Connect { player_name: String },
    Move { x: f32, y: f32, z: f32 },
    RequestChunks { center_x: i32, center_y: i32 },
    PlayerPositionUpdate(PlayerPosition),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Welcome { player_id: u32 },
    PlayerMoved { player_id: u32, x: f32, y: f32, z: f32 },
    ChunkData { x: i32, y: i32, blocks: Vec<u8> },
    PlayerPositions(Vec<(String, PlayerPosition)>),
}
