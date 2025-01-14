use std::error::Error;

use game_core::{BiomeMod, TileMod};
use macroquad::prelude::*;
use std::fs;

use crate::{save::{chunk::ChunkSave, tile::TileSave, vec2::Vec2Save, world::WorldSave}, utils::{consts::{CHUNK_PIXELS, CHUNK_SIZE, DEFAULT_TILE, TILE_SIZE, WORLD_SIZE}, system::SystemInfo}};

use super::{entity::{block::{block::{Block, BlockType}, blocks::{cactus::Cactus, tree::Tree}, textures::BlockTextures}, player::{gui::PlayerGui, player::Player}}, utils::draw::DrawBatch, world::{chunk::chunk::Chunk, generator::{biome::Biome, generator::WorldGenerator}, tile::{textures::TileTextures, tile::Tile}, world::World}};

pub struct GameState {
    pub player: Player,
    pub player_gui: PlayerGui,
    pub camera: Camera2D,
    pub world: World,
    pub generator: WorldGenerator,
    pub tile_textures: TileTextures,
    pub block_textures: BlockTextures,
    pub show_debug: bool,
    pub custom_font: Font,
}

impl GameState {
    pub async fn new(seed: u32, custom_biomes: Vec<BiomeMod>, custom_tiles: Vec<TileMod>) -> Self {
        let player = Player::new().await;
        let player_gui = PlayerGui::new().await;
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), -screen_height()));
        let mut tile_textures = Tile::load_textures().await;
        for tile in custom_tiles {
            println!("Trying to load texture for tile: {} from: {}", tile.name, tile.texture_path);
            if let Ok(texture) = load_texture(&tile.texture_path).await {
                println!("Successfully loaded texture for: {}", tile.name);
                tile_textures.custom.insert(tile.id, texture);
            } else {
                println!("Failed to load texture for: {}", tile.name);
            }
        }
    
        let block_textures = BlockTextures::load().await;
        let mut generator = WorldGenerator::new(seed, WORLD_SIZE, WORLD_SIZE);
        
        for biome in custom_biomes {
            generator.add_custom_biome(biome);
        }
        
        let mut world = World::new(WORLD_SIZE, WORLD_SIZE);
        
        for chunk in &mut world.chunks {
            *chunk = generator.generate_chunk(
                chunk.pos.0,
                chunk.pos.1,
                tile_textures.clone(),
                &block_textures
            ).await;
        }
        
        Self {
            player,
            player_gui,
            camera,
            world,
            generator,
            tile_textures,
            block_textures,
            show_debug: false,
            custom_font: load_ttf_font("assets/bonspixels.ttf").await.expect("Failed to load font"),
        }
    }
    pub async fn update(&mut self, dt: f32, system_info: &SystemInfo) {
        
        self.world.cleanup_chunks();
        self.world.update_chunks(self.player.pos, &mut self.generator, &self.tile_textures, &self.block_textures).await;
         
        let world_size = CHUNK_SIZE as f32 * 32.0 * 32.0;
        self.update_camera(world_size);
        
        let visible_chunks: Vec<&Chunk> = self.world.chunks.iter()
            .filter(|chunk| chunk.is_visible(self.camera.target, vec2(screen_width(), screen_height())))
            .collect();
            
        self.draw_chunks(&visible_chunks);
        let (tiles, blocks) = self.collect_visible_objects(&visible_chunks);
        
        self.player.update(dt, &tiles, &blocks);
        self.player.draw();
        
        if is_key_pressed(KeyCode::F3) {
            self.show_debug = !self.show_debug;
        }
        if self.show_debug {
            self.draw_debug(&tiles, system_info);
        }
        if is_key_pressed(KeyCode::F5) {
            self.save_game("world");
        }
    }

    fn update_camera(&mut self, world_size: f32) {
        self.camera.target = self.player.pos;
        
        let base_zoom = 0.0033334 / 3.0;
        
        let aspect_ratio = screen_width() / screen_height();
        self.camera.zoom = if aspect_ratio > 1.0 {
            vec2(base_zoom / aspect_ratio, base_zoom)
        } else {
            vec2(base_zoom, base_zoom * aspect_ratio)
        };
        
        set_camera(&self.camera);
    }
    fn draw_chunks(&self, chunks: &[&Chunk]) {
        let mut batch = DrawBatch::new();
        
        for chunk in chunks {
            for tile in &chunk.tiles {
                tile.draw(&chunk.tiles, CHUNK_SIZE, &mut batch);
            }
            batch.draw();
        
            for block in &chunk.blocks {
                block.draw();
            }
        }
    }

    fn collect_visible_objects<'a>(&self, chunks: &'a [&'a Chunk]) -> (Vec<&'a Tile>, Vec<&'a Box<dyn Block>>) {
        let mut tiles = Vec::new();
        let mut blocks = Vec::new();

        for chunk in chunks {
            tiles.extend(chunk.tiles.iter());
            blocks.extend(chunk.blocks.iter());
        }

        (tiles, blocks)
    }

    fn draw_debug(&self, all_tiles: &[&Tile], system_info: &SystemInfo) {
        if !self.show_debug {
            return;
        }
        
        set_default_camera();
        
        let player_pos = self.get_player_position();
        let current_tile = self.get_current_tile(all_tiles);
        
        self.draw_debug_info(player_pos, current_tile, system_info);
        
        set_camera(&self.camera);
    }
    
    fn get_player_position(&self) -> (usize, usize, usize, usize) {
        let player_tile_x = (self.player.pos.x / TILE_SIZE) as usize;
        let player_tile_y = (self.player.pos.y / TILE_SIZE) as usize;
        let player_chunk_x = (self.player.pos.x / CHUNK_PIXELS) as usize;
        let player_chunk_y = (self.player.pos.y / CHUNK_PIXELS) as usize;
        
        (player_tile_x, player_tile_y, player_chunk_x, player_chunk_y)
    }
    
    fn get_current_tile<'a>(&self, all_tiles: &'a [&'a Tile]) -> &'a Tile {
        let (tile_x, tile_y, _, _) = self.get_player_position();
        
        all_tiles.iter()
            .find(|t|
                (t.pos.x / TILE_SIZE) as usize == tile_x &&
                (t.pos.y / TILE_SIZE) as usize == tile_y
            )
            .copied()
            .unwrap_or(&DEFAULT_TILE)
    }

    fn draw_debug_info(&self, pos: (usize, usize, usize, usize), tile: &Tile, system_info: &SystemInfo) {
        let lines = [
            format!("Coords: ({:.2}, {:.2})", self.player.pos.x, self.player.pos.y),
            format!("Chunk: ({}, {})", pos.2, pos.3),
            format!("Loaded Chunks: {}", self.world.chunks.len()),
            format!("Biome: {:?}", tile.biome),
            format!("Seed: {}", self.generator.seed),
            format!("RAM Usage: {} MB", system_info.process_memory),
            format!("CPU Usage: {:.1}%", system_info.cpu_usage),
            format!("FPS: {}", system_info.fps),
        ];
    
        for (i, line) in lines.iter().enumerate() {
            draw_text(line, 10.0, 20.0 + (i as f32 * 25.0), 20.0, WHITE);
        }
    
        self.draw_performance_indicator(
            system_info.process_memory,
            system_info.cpu_usage,
            system_info.fps,
            screen_width() - 150.0,
            10.0
        );
    }
    
    fn draw_performance_indicator(&self, ram: u64, cpu: f32, fps: u32, x: f32, y: f32) {
        let ram_color = match ram {
            0..=512 => GREEN,
            513..=1024 => YELLOW,
            _ => RED,
        };
    
        let cpu_color = match cpu as i32 {
            0..=30 => GREEN,
            31..=70 => YELLOW,
            _ => RED,
        };
    
        let fps_color = match fps {
            60..=1000 => GREEN,
            30..=59 => YELLOW,
            _ => RED,
        };
    
        draw_rectangle(x, y, 50.0, 10.0, ram_color);
        draw_rectangle(x, y + 15.0, 50.0, 10.0, cpu_color);
        draw_rectangle(x, y + 30.0, 50.0, 10.0, fps_color);
    
        draw_text(&format!("{} MB", ram), x + 60.0, y + 10.0, 20.0, ram_color);
        draw_text(&format!("{:.1}% CPU", cpu), x + 60.0, y + 25.0, 20.0, cpu_color);
        draw_text(&format!("{} FPS", fps), x + 60.0, y + 40.0, 20.0, fps_color);
    }
    pub fn save_game(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let save = WorldSave {
            seed: self.generator.seed,
            player_pos: Vec2Save::from(self.player.pos),
            chunks: self.world.chunks.iter()
                .map(|chunk| ChunkSave {
                    pos: chunk.pos,
                    tiles: chunk.tiles.iter().map(TileSave::from).collect(),
                    blocks: chunk.blocks.iter().map(|block| block.to_save()).collect(),
                })
                .collect()
        };
        
        let json = serde_json::to_string(&save)?;
        fs::write(filename, json)?;
        Ok(())
    }
    pub async fn load_game(filename: &str) -> Result<Self, Box<dyn Error>> {
        let json = fs::read_to_string(filename)?;
        let save: WorldSave = serde_json::from_str(&json)?;

        let mut game = Self::new(save.seed, Vec::new(), Vec::new()).await;
        game.player.pos = Vec2::from(save.player_pos);

        game.world.chunks.clear();
        for chunk_save in save.chunks {
            let mut chunk = Chunk::new((chunk_save.pos.0, chunk_save.pos.1));
            chunk.tiles = chunk_save.tiles.iter()
                .map(|tile_save| Tile::from(tile_save.clone()))
                .collect();

            chunk.blocks = chunk_save.blocks.iter()
                .map(|block_save| -> Box<dyn Block> {
                    let tile_pos = (block_save.pos.x / 32.0) as usize;
                    let tile_biome = chunk.tiles.iter()
                        .find(|t| (t.pos.x / 32.0) as usize == tile_pos)
                        .map(|t| &t.biome)
                        .unwrap_or(&Biome::Forest);

                    match block_save.block_type {
                        BlockType::Cactus => {
                            let mut cactus = Cactus::from_save(block_save.clone());
                            cactus.set_texture(game.block_textures.cactus.clone());
                            Box::new(cactus)
                        },
                        BlockType::Tree => {
                            let mut tree = Tree::from_save(block_save.clone());
                            tree.texture_bottom = Some(game.block_textures.tree.clone());
                            tree.texture_top = Some(match tile_biome {
                                Biome::SnowForest => game.block_textures.tree_snow_top.clone(),
                                _ => game.block_textures.tree_top.clone(),
                            });
                            Box::new(tree)
                        },
                    }
                })
                .collect();

            game.world.chunks.push(chunk);
        }

        Ok(game)
    }
}