use macroquad::prelude::*;

use super::textures::ButtonTextures;

pub struct Button {
    pub text: String,
    pub pos: Vec2,
    pub width: f32,
    pub textures: ButtonTextures,
    pub hovered: bool,
}


impl Button {
    pub fn new(text: &str, pos: Vec2, textures: ButtonTextures) -> Self {
        Self {
            text: text.to_string(),
            pos,
            width: textures.start.width() + textures.end.width() + 200.0,
            textures,
            hovered: false,
        }
    }

    pub fn is_clicked(&mut self) -> bool {
        let mouse_pos = mouse_position();
        let height = self.textures.start.height();

        self.hovered = mouse_pos.0 >= self.pos.x 
            && mouse_pos.0 <= self.pos.x + self.width
            && mouse_pos.1 >= self.pos.y 
            && mouse_pos.1 <= self.pos.y + height;

        self.hovered && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn draw(&self) {
        let color = if self.hovered { WHITE } else { GRAY };
        let height = self.textures.start.height();
        let start_width = self.textures.start.width();
        let end_width = self.textures.end.width();
        let middle_width = self.width - (start_width + end_width);
        let middle_segment_width = self.textures.middle[0].width();

        draw_texture(&self.textures.start, self.pos.x, self.pos.y, color);

        let mut x = self.pos.x + start_width;
        let segments = (middle_width / middle_segment_width).ceil() as usize;
        for i in 0..segments {
            let texture = &self.textures.middle[i % self.textures.middle.len()];
            draw_texture(texture, x, self.pos.y, color);
            x += middle_segment_width;
        }

        draw_texture(&self.textures.end, self.pos.x + self.width - end_width, self.pos.y, color);

        let text_size = 20.0; 
        let text_pos = vec2(
            self.pos.x + (self.width - measure_text(&self.text, None, text_size as u16, 1.0).width) / 2.0,
            self.pos.y + (height - text_size / 2.0)
        );
        draw_text(&self.text, text_pos.x, text_pos.y, text_size, BLACK);
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.textures.start.height()
    }
}