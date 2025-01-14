use serde::{Deserialize, Serialize};

use crate::game::entity::block::block::BlockType;

use super::vec2::Vec2Save;

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockSave {
    pub pos: Vec2Save,
    pub block_type: BlockType,
}