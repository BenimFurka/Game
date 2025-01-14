use serde::{Deserialize, Serialize};

use super::{chunk::ChunkSave, vec2::Vec2Save};

#[derive(Serialize, Deserialize)]
pub struct WorldSave {
    pub seed: u32,
    pub player_pos: Vec2Save,
    pub chunks: Vec<ChunkSave>,
}