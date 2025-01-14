use crate::{game::{entity::block::block::Block, world::tile::tile::Tile}, utils::consts::{CHUNK_PIXELS, CHUNK_SIZE, TILE_SIZE}};
use macroquad::prelude::*;
pub struct Chunk {
    pub tiles: Vec<Tile>,
    pub blocks: Vec<Box<dyn Block>>,
    pub pos: (usize, usize),
    bounds: (Vec2, Vec2),
}

impl Chunk {
    pub fn new(pos: (usize, usize)) -> Self {
        let min = vec2(
            pos.0 as f32 * CHUNK_PIXELS,
            pos.1 as f32 * CHUNK_PIXELS
        );
        let max = min + vec2(CHUNK_PIXELS, CHUNK_PIXELS);

        println!("NEW: chunk {:?} bounds: {:?} to {:?}", pos, min, max);

        Self {
            tiles: Vec::new(),
            blocks: Vec::new(),
            pos,
            bounds: (min, max),
        }
    }

    pub fn is_visible(&self, camera_pos: Vec2, screen_size: Vec2) -> bool {
        let chunk_min = Vec2::new(
            self.pos.0 as f32 * CHUNK_SIZE as f32 * TILE_SIZE,
            self.pos.1 as f32 * CHUNK_SIZE as f32 * TILE_SIZE
        );
        
        let chunk_max = chunk_min + Vec2::new(
            CHUNK_SIZE as f32 * TILE_SIZE,
            CHUNK_SIZE as f32 * TILE_SIZE
        );
        
        let screen_min = camera_pos - screen_size / 2.0;
        let screen_max = camera_pos + screen_size / 2.0;
        
        !(chunk_max.x < screen_min.x ||
        chunk_min.x > screen_max.x ||
        chunk_max.y < screen_min.y ||
        chunk_min.y > screen_max.y)
    }
}