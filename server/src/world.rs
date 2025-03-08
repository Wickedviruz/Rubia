use noise::{NoiseFn, Perlin};
use crate::messages::ServerMessage;

pub const CHUNK_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub blocks: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn generate(x: i32, y: i32) -> Self {
        let perlin = Perlin::default();
        let mut blocks = [[0u8; CHUNK_SIZE]; CHUNK_SIZE];

        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let world_x = (x * CHUNK_SIZE as i32 + i as i32) as f64;
                let world_y = (y * CHUNK_SIZE as i32 + j as i32) as f64;

                // Skala och generera höjden (höjd mellan 1-10)
                let height = ((perlin.get([world_x / 20.0, world_y / 20.0]) + 1.0) * 5.0) as u8;

                blocks[i][j] = height.clamp(1, 10);
            }
        }

        Self { x, y, blocks }
    }

    pub fn to_flat_vec(&self) -> Vec<u8> {
        self.blocks.iter().flatten().copied().collect()
    }
}