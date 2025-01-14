use macroquad::prelude::*;

pub struct DrawBatch {
    textures: Vec<(Texture2D, Vec2, f32, Option<Vec2>)>, 
}

impl DrawBatch {
    pub fn new() -> Self {
        Self {
            textures: Vec::new()
        }
    }

    pub fn add(&mut self, texture: Texture2D, pos: Vec2, size: f32, dest_size: Option<Vec2>) {
        self.textures.push((texture, pos, size, dest_size));
    }

    pub fn draw(&mut self) {
        for (texture, pos, size, dest_size) in &self.textures {
            draw_texture_ex(
                texture,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: *dest_size,
                    ..Default::default()
                }
            );
        }
        self.textures.clear();
    }
}

pub fn distance_squared(a: (usize, usize), b: (usize, usize)) -> f32 {
    let dx = a.0 as f32 - b.0 as f32;
    let dy = a.1 as f32 - b.1 as f32;
    dx * dx + dy * dy
}