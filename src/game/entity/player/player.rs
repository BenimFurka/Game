use macroquad::prelude::*;

use crate::game::{entity::block::block::Block, world::tile::{state::TileState, tile::Tile}};

use super::{direction::Direction, textures::PlayerTextures};

pub struct Player {
    pub pos: Vec2,
    pub size: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub direction: Direction,
    pub is_moving: bool,
    animation_frame: i32,
    animation_timer: f32,
    textures: PlayerTextures
}

impl Player {
    pub async fn new() -> Self {
        let textures = PlayerTextures {
            idle_up: load_texture("assets/textures/entities/player/idle_up.png").await.unwrap(),
            idle_down: load_texture("assets/textures/entities/player/idle_down.png").await.unwrap(),
            idle_left: load_texture("assets/textures/entities/player/idle_left.png").await.unwrap(),
            idle_right: load_texture("assets/textures/entities/player/idle_right.png").await.unwrap(),
            walk_up: [
                load_texture("assets/textures/entities/player/walk_up_1.png").await.unwrap(),
                load_texture("assets/textures/entities/player/walk_up_2.png").await.unwrap(),
            ],
            walk_down: [
                load_texture("assets/textures/entities/player/walk_down_1.png").await.unwrap(),
                load_texture("assets/textures/entities/player/walk_down_2.png").await.unwrap(),
            ],
            walk_left: [
                load_texture("assets/textures/entities/player/walk_left_1.png").await.unwrap(),
                load_texture("assets/textures/entities/player/walk_left_2.png").await.unwrap(),
            ],
            walk_right: [
                load_texture("assets/textures/entities/player/walk_right_1.png").await.unwrap(),
                load_texture("assets/textures/entities/player/walk_right_2.png").await.unwrap(),
            ],
        };
        Player {
            pos: vec2(512.0 * 32.0, 512.0 * 32.0),
            size: 32.0,
            direction: Direction::Down,
            is_moving: false,
                        
            hp: 50,
            max_hp: 50,
            animation_frame: 0,
            animation_timer: 0.0,
            textures
        }
    }
    pub fn update(&mut self, dt: f32, tiles: &[&Tile], blocks: &[&Box<dyn Block>]) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        
        if is_key_down(KeyCode::W) { 
            dy -= 1.0;}
        if is_key_down(KeyCode::S) { dy += 1.0; }
        if is_key_down(KeyCode::A) { dx -= 1.0; }
        if is_key_down(KeyCode::D) { dx += 1.0; }
        
        if dx != 0.0 || dy != 0.0 {
            if (dx as f32).abs() > (dy as f32).abs() {
                self.direction = if dx > 0.0 { Direction::Right } else { Direction::Left };
            } else {
                self.direction = if dy > 0.0 { Direction::Down } else { Direction::Up };
            }
        }
        
        if dx != 0.0 && dy != 0.0 {
            let len = ((dx * dx + dy * dy) as f32).sqrt();
            dx /= len;
            dy /= len;
        }
    
        let base_speed = 2.0;
        dx *= base_speed;
        dy *= base_speed;
        
        let next_x = self.pos.x + dx;
        let next_y = self.pos.y + dy;
        
        let mut can_move_x = true;
        let mut can_move_y = true;
        
        for block in blocks.iter().filter(|block| {
            let dx = (block.get_position().x - self.pos.x).abs();
            let dy = (block.get_position().y - self.pos.y).abs();
            dx < 64.0 && dy < 64.0
        }) {
            let buffer = 2.0; 

            if next_x < block.get_position().x + block.get_size().x - buffer &&
                next_x + self.size > block.get_position().x + buffer &&
                self.pos.y < block.get_position().y + block.get_size().y - buffer &&
                self.pos.y + self.size > block.get_position().y + buffer {
                can_move_x = false;
            }

            if self.pos.x < block.get_position().x + block.get_size().x - buffer &&
                self.pos.x + self.size > block.get_position().x + buffer &&
                next_y < block.get_position().y + block.get_size().y - buffer &&
                next_y + self.size > block.get_position().y + buffer {
                can_move_y = false;
            }
        }
                
        let tile_x = (self.pos.x / 32.0) as usize;
        let tile_y = (self.pos.y / 32.0) as usize;
        
        if let Some(current_tile) = tiles.iter().find(|t|
            (t.pos.x / 32.0) as usize == tile_x &&
            (t.pos.y / 32.0) as usize == tile_y) {
            let speed_multiplier = match current_tile.state {
                TileState::Water => 0.5,
                TileState::Sand => 0.9,
                TileState::SnowGrass => 0.9,
                _ => 1.0,
            };
            
            if can_move_x { self.pos.x += dx * speed_multiplier; }
            if can_move_y { self.pos.y += dy * speed_multiplier; }
        }
            
        if dx != 0.0 || dy != 0.0 {
            self.is_moving = true;
            self.animation_timer += dt;
            if self.animation_timer > 0.15 {
                self.animation_timer = 0.0;
                self.animation_frame = (self.animation_frame + 1) % 3;
            }
        } else {
            self.is_moving = false;
            self.animation_frame = 1;
        }
    }
    pub fn draw(&self) {
        let texture = if self.is_moving {
            match (&self.direction, self.animation_frame) {
                (Direction::Up, 0) => &self.textures.walk_up[0],
                (Direction::Up, 1) => &self.textures.idle_up,
                (Direction::Up, 2) => &self.textures.walk_up[1],
                (Direction::Down, 0) => &self.textures.walk_down[0],
                (Direction::Down, 1) => &self.textures.idle_down,
                (Direction::Down, 2) => &self.textures.walk_down[1],
                (Direction::Left, 0) => &self.textures.walk_left[0],
                (Direction::Left, 1) => &self.textures.idle_left,
                (Direction::Left, 2) => &self.textures.walk_left[1],
                (Direction::Right, 0) => &self.textures.walk_right[0],
                (Direction::Right, 1) => &self.textures.idle_right,
                (Direction::Right, 2) => &self.textures.walk_right[1],
                _ => &self.textures.idle_down,
            }
        } else {
            match self.direction {
                Direction::Up => &self.textures.idle_up,
                Direction::Down => &self.textures.idle_down,
                Direction::Left => &self.textures.idle_left,
                Direction::Right => &self.textures.idle_right,
            }
        };
        
        draw_texture_ex(
            texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size, self.size)),
                ..Default::default()
            }
        );
    }
}
