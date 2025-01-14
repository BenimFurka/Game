use macroquad::prelude::*;

use crate::{game::entity::block::block::{Block, BlockType}, save::{block::BlockSave, vec2::Vec2Save}};

pub struct Cactus {
    pub pos: Vec2,
    pub size: Vec2,
    texture: Option<Texture2D>,
}

impl Block for Cactus {
    fn get_position(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn draw(&self) {
        if let Some(tex) = &self.texture {
            draw_texture_ex(
                tex,
                self.pos.x,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(self.size),
                    ..Default::default()
                }
            );
        }
    }

    fn set_texture(&mut self, texture: Texture2D) {
        self.texture = Some(texture);
    }
    fn to_save(&self) -> BlockSave {
        BlockSave {
            pos: Vec2Save::from(self.pos),
            block_type: BlockType::Cactus,
        }
    }

    fn from_save(save: BlockSave) -> Self {
        Self {
            pos: Vec2::from(save.pos),
            size: vec2(32.0,32.0),
            texture: None,
        }
    }
    
    fn set_position(&mut self, pos: Vec2) {
        self.pos = pos;
    }
}

impl Cactus {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            size: vec2(32.0, 32.0),
            texture: None,
        }
    }
}