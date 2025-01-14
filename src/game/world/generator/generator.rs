use game_core::{BiomeConditions, BiomeMod};
use noise::{NoiseFn, Perlin};
use macroquad::prelude::*;
use crate::{game::{entity::block::{block::{Block, BlockCache, BlockType}, blocks::{cactus::Cactus, tree::Tree}, textures::BlockTextures}, world::{chunk::chunk::Chunk, tile::{state::TileState, textures::TileTextures, tile::Tile}}}, utils::consts::{BLOCK_CACHE, CHUNK_SIZE}};

use super::{biome::{Biome, BiomeCache}, noise::{NoiseCache, NoiseType}};

pub struct WorldGenerator {
    perlin: Perlin,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub scale: f64,
    pub moisture_scale: f64,
    custom_biomes: Vec<BiomeMod>,
        
    noise_cache: NoiseCache,
    biome_cache: BiomeCache,
    block_cache: BlockCache,
    
    octaves: usize,
    persistence: f64, 
    lacunarity: f64,
}

impl WorldGenerator {
    pub fn new(seed: u32, width: usize, height: usize) -> Self {
        Self {
        perlin: Perlin::new(seed),
        seed,
        width,
        height,
        scale: 0.05,
        moisture_scale: 0.03,
        custom_biomes: Vec::new(),
        noise_cache: NoiseCache::new(),
        
        biome_cache: BiomeCache::new(),
        block_cache: BlockCache::new(BLOCK_CACHE),
        
        octaves: 2,
        persistence: 0.3,  
        lacunarity: 2.0,
        }
    }
    pub fn generate_chunk_data(&mut self, chunk: &mut Chunk, tile_textures: TileTextures, block_textures: &BlockTextures) {
        let height_map = self.generate_chunk_height_map(chunk.pos.0, chunk.pos.1);
        chunk.tiles.clear();
        chunk.blocks.clear();

        chunk.tiles = self.generate_chunk_tiles(chunk.pos.0, chunk.pos.1, &height_map, tile_textures);
        chunk.blocks = self.generate_chunk_blocks(chunk.pos.0, chunk.pos.1, &height_map, block_textures);
    }
       
    pub fn generate_chunk_height_map(&mut self, chunk_x: usize, chunk_y: usize) -> Vec<Vec<(f64, f64, f64)>> {
        let mut map = vec![vec![(0.0, 0.0, 0.0); CHUNK_SIZE]; CHUNK_SIZE];
    
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let world_x = chunk_x * CHUNK_SIZE + x;
                let world_y = chunk_y * CHUNK_SIZE + y;
    
                let height = self.generate_noise_with_octaves(world_x as f64, world_y as f64, NoiseType::Height);
                let moisture = self.generate_noise_with_octaves(world_x as f64, world_y as f64, NoiseType::Moisture);
                let temperature = self.generate_noise_with_octaves(world_x as f64, world_y as f64, NoiseType::Temperature);
    
                map[y][x] = (
                    (height + 1.0) / 2.0,  
                    (moisture + 1.0) / 2.0,
                    (temperature + 1.0) / 2.0
                );
            }
        }
    
        map
    }
    fn generate_noise_with_octaves(&self, x: f64, y: f64, noise_type: NoiseType) -> f64 {
        let (offset_x, offset_y) = match noise_type {
            NoiseType::Height => (0.0, 0.0),
            NoiseType::Moisture => (10000.0, 10000.0),
            NoiseType::Temperature => (20000.0, 20000.0),
            NoiseType::BlockChance => (30000.0, 30000.0),
        };
        
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut noise_value = 0.0;
        
        for _ in 0..self.octaves {
            let sample_x = (x + offset_x) * self.scale * frequency;
            let sample_y = (y + offset_y) * self.scale * frequency;
            
            let noise = self.perlin.get([sample_x, sample_y]);
            
            noise_value += noise * amplitude;
            amplitude *= self.persistence;
            frequency *= self.lacunarity;
        }
        
        noise_value
    }
    
    pub async fn generate_chunk(&mut self, chunk_x: usize, chunk_y: usize, tile_textures: TileTextures, block_textures: &BlockTextures) -> Chunk {
        let height_map = self.generate_chunk_height_map(chunk_x, chunk_y);
        let tiles = self.generate_chunk_tiles(chunk_x, chunk_y, &height_map, tile_textures);
        let blocks = self.generate_chunk_blocks(chunk_x, chunk_y, &height_map, block_textures);
        
        let mut chunk = Chunk::new((chunk_x, chunk_y));
        chunk.tiles = tiles;
        chunk.blocks = blocks;
        chunk
    }
    fn generate_chunk_tiles(&mut self, chunk_x: usize, chunk_y: usize, height_map: &[Vec<(f64, f64, f64)>], tile_textures: TileTextures) -> Vec<Tile> {
        let mut tiles = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE);
        
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let world_x = chunk_x * CHUNK_SIZE + x;
                let world_y = chunk_y * CHUNK_SIZE + y;
                
                let (height, moisture, temperature) = height_map[y][x];
                
                
                let biome = if let Some(cached_biome) = self.biome_cache.biomes.get(&(world_x, world_y)) {
                    cached_biome.clone()
                } else {
                    let new_biome = self.get_biome(height, moisture, temperature);
                    self.biome_cache.biomes.insert((world_x, world_y), new_biome.clone());
                    new_biome
                };
                
                let state = if let Some(cached_state) = self.biome_cache.tile_states.get(&biome) {
                    cached_state.clone()
                } else {
                    let new_state = self.get_tile_state(biome.clone());
                    self.biome_cache.tile_states.insert(biome.clone(), new_state.clone());
                    new_state
                };
                
                let mut tile = Tile::new(
                    world_x as f32 * 32.0,
                    world_y as f32 * 32.0,
                    Some(tile_textures.clone())
                );
                tile.state = state;
                tile.biome = biome;
                tiles.push(tile);
            }
        }
        tiles
    }
    fn generate_chunk_blocks(&mut self, chunk_x: usize, chunk_y: usize, height_map: &[Vec<(f64, f64, f64)>], textures: &BlockTextures) -> Vec<Box<dyn Block>> {
        let mut blocks: Vec<Box<dyn Block>> = Vec::new();
        
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let world_x = chunk_x * CHUNK_SIZE + x;
                let world_y = chunk_y * CHUNK_SIZE + y;
                
                let (height, moisture, temperature) = height_map[y][x];
                let biome = self.get_biome(height, moisture, temperature);
                let pos = vec2(world_x as f32 * 32.0, world_y as f32 * 32.0);
                
                let block_chance = self.noise_cache.get_or_generate(
                    (world_x, world_y),
                    NoiseType::BlockChance,
                    &self.perlin,
                    self.scale,
                    self.moisture_scale
                );
                
                match biome {
                    Biome::Forest | Biome::SnowForest if block_chance > 0.70 => {
                        if let Some(mut block) = self.block_cache.get_block(BlockType::Tree) {
                            block.set_position(pos);
                            block.set_texture(if matches!(biome, Biome::SnowForest) {
                                textures.tree_snow_top.clone()
                            } else {
                                textures.tree_top.clone()
                            }); 
                            blocks.push(block);
                        } else {
                            blocks.push(Box::new(Tree::new(pos, textures, biome)));
                        }
                    },
                    Biome::Desert if block_chance > 0.8 => {
                        if let Some(mut block) = self.block_cache.get_block(BlockType::Cactus) {
                            block.set_position(pos);
                            block.set_texture(textures.cactus.clone());
                            blocks.push(block);
                        } else {
                            let mut cactus = Cactus::new(pos);
                            cactus.set_texture(textures.cactus.clone());
                            blocks.push(Box::new(cactus));
                        }
                    },
                    _ => {}
                }
            }
        }
        blocks
    }

    fn matches_conditions(&self, values: &(f64, f64, f64), conditions: &BiomeConditions) -> bool {
        let (height, moisture, temperature) = *values;
        conditions.height_range.map_or(true, |(min, max)| height >= min && height <= max)
            && conditions.moisture_range.map_or(true, |(min, max)| moisture >= min && moisture <= max)
            && conditions.temperature_range.map_or(true, |(min, max)| temperature >= min && temperature <= max)
    }

    pub fn get_biome(&self, height: f64, moisture: f64, temperature: f64) -> Biome {
        for biome_mod in &self.custom_biomes {
            if self.matches_conditions(&(height, moisture, temperature), &biome_mod.conditions) {
                return Biome::Custom(biome_mod.id.clone());
            }
        }
        if height < 0.3 {
            return Biome::River;
        } else if height < 0.4 && temperature > 0.3 {
            return Biome::Beach;
        }

        if temperature < 0.3 {
            if moisture > 0.6 {
                return Biome::SnowForest;
            } else {
                return Biome::SnowPlains;
            }
        }

        if temperature > 0.8 {
            if moisture < 0.3 {
                return Biome::Desert;
            } }
        if moisture > 0.6 {
            Biome::Forest
        } else {
            Biome::Plains
        }


    }

    pub fn get_tile_state(&self, biome: Biome) -> TileState {
        match biome {
            Biome::River => TileState::Water,
            Biome::Beach => TileState::Sand,
            Biome::Plains => TileState::Grass,
            Biome::Forest => TileState::Grass,
            Biome::SnowPlains => TileState::SnowGrass,
            Biome::SnowForest => TileState::SnowGrass,
            Biome::Desert => TileState::Sand,
            Biome::Custom(ref id) => {
                if let Some(biome_mod) = self.custom_biomes.iter().find(|b| b.id == *id) {
                    if biome_mod.tile_state.starts_with("Custom(") {
                        TileState::Custom(biome_mod.tile_state.clone())
                    } else {
                        match biome_mod.tile_state.as_str() {
                            "Water" => TileState::Water,
                            "Sand" => TileState::Sand,
                            "Grass" => TileState::Grass,
                            "SnowGrass" => TileState::SnowGrass,
                            "Tilled" => TileState::Tilled,
                            _ => TileState::Grass
                        }
                    }
                } else {
                    TileState::Grass
                }
            }
        }

    }
    pub fn add_custom_biome(&mut self, biome: BiomeMod) {
        self.custom_biomes.push(biome);
    }
}
