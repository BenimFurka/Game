use serde::{Deserialize, Serialize};

use super::{block::BlockSave, tile::TileSave};



#[derive(Serialize, Deserialize)]
pub struct ChunkSave {
    pub pos: (usize, usize),
    pub tiles: Vec<TileSave>,
    pub blocks: Vec<BlockSave>,
}