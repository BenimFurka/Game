use crate::{game::{entity::block::textures::BlockTextures, utils::draw::distance_squared}, utils::consts::{CHUNK_PIXELS, CHUNK_SIZE, MAX_CHUNKS, RENDER_DISTANCE}};

use super::{chunk::{chunk::Chunk, pool::ChunkPool}, generator::generator::WorldGenerator, tile::textures::TileTextures};
use macroquad::prelude::*;
pub struct World {
    pub chunks: Vec<Chunk>,
    pub width: usize,
    pub height: usize,
    pub chunk_pool: ChunkPool,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let chunks_x = width / CHUNK_SIZE;
        let chunks_y = height / CHUNK_SIZE;
        let pool_size = MAX_CHUNKS;

        println!("Creating world {}x{} with {}x{} chunks",
        width, height, chunks_x, chunks_y);

        Self {
            chunks: Vec::new(),
            width,
            height,
            chunk_pool: ChunkPool::new(pool_size),
        }
    }

    pub fn cleanup_chunks(&mut self) {

        if self.chunks.len() > MAX_CHUNKS {
            self.chunks.sort_by(|a, b| {
                let center_x = self.width / 2;
                let center_y = self.height / 2;

                let a_dist = distance_squared(a.pos, (center_x, center_y));
                let b_dist = distance_squared(b.pos, (center_x, center_y));

                b_dist.partial_cmp(&a_dist).unwrap()
            });
            while self.chunks.len() > MAX_CHUNKS {
                if let Some(chunk) = self.chunks.pop() {
                    self.chunk_pool.return_chunk(chunk);
                }
            }
        }
    }
    pub async fn update_chunks(&mut self, player_pos: Vec2, generator: &mut WorldGenerator, tile_textures: &TileTextures, block_textures: &BlockTextures) {
        let player_chunk_x = (player_pos.x / CHUNK_PIXELS) as usize;
        let player_chunk_y = (player_pos.y / CHUNK_PIXELS) as usize;
        let render_distance = RENDER_DISTANCE;
        
        self.remove_far_chunks(player_chunk_x, player_chunk_y, render_distance);
        
        self.create_nearby_chunks(
            player_chunk_x,
            player_chunk_y,
            render_distance,
            generator,
            tile_textures,
            block_textures
        ).await;
    }
    
    fn remove_far_chunks(&mut self, player_x: usize, player_y: usize, distance: f32) {
        let to_remove: Vec<_> = self.chunks.iter()
        .filter(|chunk| {
            let dx = chunk.pos.0 as isize - player_x as isize;
            let dy = chunk.pos.1 as isize - player_y as isize;
            ((dx * dx + dy * dy) as f32).sqrt() > distance
        })
            .map(|chunk| chunk.pos)
            .collect();
            
        for pos in &to_remove {
            if let Some(idx) = self.chunks.iter().position(|c| c.pos == *pos) {
                let chunk = self.chunks.remove(idx);
                self.chunk_pool.return_chunk(chunk);
            }
        }
    }
    
    async fn create_nearby_chunks(&mut self, player_x: usize, player_y: usize, distance: f32, 
        generator: &mut WorldGenerator, tile_textures: &TileTextures, block_textures: &BlockTextures) {
        for dy in -2..=2 {
            for dx in -2..=2 {
                if ((dx * dx + dy * dy) as f32).sqrt() <= distance {
                    let chunk_x = player_x as isize + dx;
                    let chunk_y = player_y as isize + dy;
                    
                    if chunk_x >= 0 && chunk_y >= 0 {
                        self.create_chunk_if_needed(
                            chunk_x as usize,
                            chunk_y as usize,
                            generator,
                            tile_textures,
                            block_textures
                        ).await;
                    }
                }
            }
        }
    }
    async fn create_chunk_if_needed(&mut self, x: usize, y: usize, 
        generator: &mut WorldGenerator, 
        tile_textures: &TileTextures, 
        block_textures: &BlockTextures) {
            
            if !self.chunks.iter().any(|c| c.pos.0 == x && c.pos.1 == y) {
                if let Some(mut chunk) = self.chunk_pool.get_chunk() {
                    chunk.pos = (x, y);
                    generator.generate_chunk_data(&mut chunk, tile_textures.clone(), block_textures);
                    self.chunks.push(chunk);
                } else {
                    let new_chunk = generator.generate_chunk(
                        x,
                        y,
                        tile_textures.clone(),
                        block_textures
                    ).await;
                    self.chunks.push(new_chunk);
                }
            }
        }
}
