use macroquad::prelude::*;

use crate::{game::{entity::block::{block::{Block, BlockType}, textures::BlockTextures}, world::generator::biome::Biome}, save::{block::BlockSave, vec2::Vec2Save}};

pub struct Tree {
    pub pos: Vec2,
    pub size: Vec2,
    pub texture_bottom: Option<Texture2D>,
    pub texture_top: Option<Texture2D>,
}

impl Block for Tree {
    fn get_position(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn draw(&self) {
        if let Some(tex_bottom) = &self.texture_bottom {
            draw_texture_ex(
                tex_bottom,
                self.pos.x,
                self.pos.y + self.size.y/2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.size.x, self.size.y/2.0)),
                    ..Default::default()
                }
            );
        }

        if let Some(tex_top) = &self.texture_top {
            draw_texture_ex(
                tex_top,
                self.pos.x,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.size.x, self.size.y/2.0)),
                    ..Default::default()
                }
            );
        }
    }

    fn set_texture(&mut self, texture: Texture2D) {
        self.texture_bottom = Some(texture);
    }
    fn to_save(&self) -> BlockSave {
        BlockSave {
            pos: Vec2Save::from(self.pos),
            block_type: BlockType::Tree,
        }
    }

    fn from_save(save: BlockSave) -> Self {
        Self {
            pos: Vec2::from(save.pos),
            size: vec2(32.0,32.0),
            texture_top: None,
            texture_bottom: None,
        }
    }
    
    fn set_position(&mut self, pos: Vec2) {
        self.pos = pos;
    }
}

impl Tree {
    pub fn new(pos: Vec2, textures: &BlockTextures, biome: Biome) -> Self {
        Self {
            pos,
            size: vec2(32.0, 64.0),
            texture_bottom: Some(textures.tree.clone()),
            texture_top: Some(match biome {
            Biome::Forest => textures.tree_top.clone(),
            Biome::SnowForest => textures.tree_snow_top.clone(),
            _ => textures.tree_top.clone(),
            }),
        }
    }
}