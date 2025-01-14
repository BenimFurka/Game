use macroquad::texture::Texture2D;

pub struct PlayerTextures {
    pub idle_up: Texture2D,
    pub idle_down: Texture2D,
    pub idle_left: Texture2D,
    pub idle_right: Texture2D,
    pub walk_up: [Texture2D; 2],
    pub walk_down: [Texture2D; 2],
    pub walk_left: [Texture2D; 2],
    pub walk_right: [Texture2D; 2],
}
