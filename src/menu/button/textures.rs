use macroquad::prelude::*;

#[derive(Clone)]
pub struct ButtonTextures {
    pub start: Texture2D,
    pub middle: Vec<Texture2D>,
    pub end: Texture2D,
}

impl Default for ButtonTextures {
    fn default() -> Self {
        Self {
            start: Texture2D::empty(),
            middle: vec![],
            end: Texture2D::empty(),
        }
    }
}
