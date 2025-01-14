use macroquad::prelude::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::save::block::BlockSave;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub enum BlockType {
    Cactus,
    Tree,
}

pub struct BlockCache {
    trees: Vec<Box<dyn Block>>,
    cacti: Vec<Box<dyn Block>>,
    textures: HashMap<BlockType, Texture2D>,
    max_pool_size: usize,
}

impl BlockCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            trees: Vec::with_capacity(max_size),
            cacti: Vec::with_capacity(max_size),
            textures: HashMap::new(),
            max_pool_size: max_size,
        }
    }

    pub fn get_block(&mut self, block_type: BlockType) -> Option<Box<dyn Block>> {
        match block_type {
            BlockType::Tree => self.trees.pop(),
            BlockType::Cactus => self.cacti.pop(),
        }
    }
}

pub trait Block {
    fn get_position(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn draw(&self);
    fn set_texture(&mut self, texture: Texture2D);
    fn set_position(&mut self, pos: Vec2);
    fn to_save(&self) -> BlockSave;
    fn from_save(save: BlockSave) -> Self where Self: Sized;
}
