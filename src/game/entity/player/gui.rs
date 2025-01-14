use macroquad::prelude::*;

use super::player::Player;

pub struct PlayerGui {
    hearts_full: Texture2D,
    hearts_empty: Texture2D,
    hearts_half: Texture2D,
}
    
impl PlayerGui {
    pub async fn new() -> Self {
        Self {
            hearts_full: load_texture("assets/textures/gui/hearts_full.png").await.unwrap(),
            hearts_empty: load_texture("assets/textures/gui/hearts_empty.png").await.unwrap(),
            hearts_half: load_texture("assets/textures/gui/hearts_half.png").await.unwrap(),
        }
    }
    
    pub fn draw_hearts(&self, player: &Player) {
        let heart_size = 20.0;
        let hearts_count = player.max_hp / 4;
        let start_x = 10.0;
        let start_y = screen_height() - heart_size - 10.0; 
        
        for i in 0..hearts_count {
            let x = start_x + (i as f32 * (heart_size + 2.0));
            let y = start_y;
            
            draw_texture_ex(
                &self.hearts_empty,
                x, y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(heart_size, heart_size)),
                    ..Default::default()
                }
            );
            
            let current_hp = player.hp - (i * 2);
            if current_hp >= 2 {
                draw_texture_ex(
                    &self.hearts_full,
                    x, y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(heart_size, heart_size)),
                        ..Default::default()
                    }
                );
            } else if current_hp == 1 {
                draw_texture_ex(
                    &self.hearts_half,
                    x, y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(heart_size, heart_size)),
                        ..Default::default()
                    }
                );
            }
        }
    }
}