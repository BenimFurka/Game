use crate::{game::{entity::block::textures::BlockTextures, utils::draw::DrawBatch, world::{generator::generator::WorldGenerator, tile::tile::Tile, world::World}}, utils::{consts::{CHUNK_SIZE, MENU_WORLD_SIZE, TILE_SIZE}, generate_seed::generate_seed, system::SystemInfo}};
use macroquad::prelude::*;

use super::button::button::Button;
use super::button::textures::ButtonTextures;

#[derive(Debug, Clone, Copy)]
pub enum MenuAction {
    None,
    NewGame,
    LoadGame,
    Exit,
}

pub struct Menu {
    background_world: World,
    buttons: Vec<Button>,
    seed: u32,
    pub camera: Camera2D,
    pub is_debug: bool,
    button_textures: ButtonTextures,
    system_info: SystemInfo,
    state: MenuState,
    //world_settings: Option<WorldSettings>, 
    //available_saves: Vec<SaveInfo>,
}

pub enum MenuState {
    Main,
    NewGame,
    LoadGame
}
/*pub struct WorldSettings {
    seed: u32,
    world_size: usize,
}

pub struct SaveInfo {
    name: String,
    date: String,
    seed: u32,
}    */
impl Menu {
    pub async fn new() -> Self {
        let seed = generate_seed();
        let tile_textures = Tile::load_textures().await;
        let block_textures = BlockTextures::load().await;

        let mut background_world = World::new(MENU_WORLD_SIZE, MENU_WORLD_SIZE);
        let mut generator = WorldGenerator::new(seed, MENU_WORLD_SIZE, MENU_WORLD_SIZE);

        let chunks_x = MENU_WORLD_SIZE / CHUNK_SIZE as usize;
        let chunks_y = MENU_WORLD_SIZE / CHUNK_SIZE as usize;

        for y in 0..chunks_y {
            for x in 0..chunks_x {
                let chunk = generator.generate_chunk(x, y, tile_textures.clone(), &block_textures).await;
                background_world.chunks.push(chunk);
            }
        }

        let camera = Camera2D::from_display_rect(Rect::new(
            MENU_WORLD_SIZE as f32 / 2.0 * TILE_SIZE - screen_width() / 2.0,
            MENU_WORLD_SIZE as f32 / 2.0 * TILE_SIZE,
            screen_width(),
            -screen_height()
        ));

        Self {
            background_world,
            buttons: Vec::new(),
            seed,
            button_textures: ButtonTextures::default(),
            camera,
            is_debug: false,
            system_info: SystemInfo::new(),
            state: MenuState::Main,
            //world_settings: None,
            //available_saves: Vec::new()
        }
    }
    pub fn update(&mut self) -> MenuAction {
        for button in &mut self.buttons {
            if button.is_clicked() {
                return match button.text.as_str() {
                    "NEW GAME" => MenuAction::NewGame,
                    "EXIT" => MenuAction::Exit,
                    _ => MenuAction::None,
                };
            }
        }

        self.system_info.update();
        if is_key_pressed(KeyCode::F3) {
            self.is_debug = !self.is_debug;
        }
        MenuAction::None
    }

    fn draw_debug_info(&self, system_info: &SystemInfo) {
        let lines = [
            format!("Seed: {}", self.seed),
            format!("RAM Usage: {} MB", system_info.process_memory),
            format!("FPS: {:.0}", get_fps()),
        ];

        for (i, line) in lines.iter().enumerate() {
            draw_text(line, 10.0, 20.0 + (i as f32 * 25.0), 20.0, WHITE);
        }

    }

    pub fn draw(&self) {
        set_camera(&self.camera);

        // Рисуем фоновый мир
        let mut batch = DrawBatch::new();
        for chunk in &self.background_world.chunks {
            for tile in &chunk.tiles {
                tile.draw(&chunk.tiles, CHUNK_SIZE, &mut batch);
            }
            batch.draw();

            for block in &chunk.blocks {
                block.draw();
            }
        }

        set_default_camera();

        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));

        let title = "Game";
        let title_size = 40.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(
            title, 
            (screen_width() - title_width) / 2.0, 
            100.0, 
            title_size, 
            WHITE
        );

        for button in &self.buttons {
            button.draw();
        }
        if self.is_debug { self.draw_debug_info(&self.system_info);}
    }

    pub async fn init(&mut self) {
        self.button_textures = ButtonTextures {
            start: load_texture("assets/textures/gui/button_start.png").await.unwrap(),
            middle: vec![load_texture("assets/textures/gui/button_middle.png").await.unwrap()],
            end: load_texture("assets/textures/gui/button_end.png").await.unwrap(),
        };

        let button_y = 150.0;
        let button_spacing = 60.0;
        let button_width = 200.0;

        self.buttons = vec![
            Button::new("NEW GAME", vec2(screen_width() / 2.0 - button_width / 2.0, button_y), self.button_textures.clone()),
            //Button::new("LOAD", vec2(screen_width() / 2.0 - button_width / 2.0, button_y + button_spacing), self.button_textures.clone()),
            //Button::new("SETTINGS", vec2(screen_width() / 2.0 - button_width / 2.0, button_y + button_spacing * 2.0), self.button_textures.clone()),
            //Button::new("CONTROLS", vec2(screen_width() / 2.0 - button_width / 2.0, button_y + button_spacing * 3.0), self.button_textures.clone()),
            //Button::new("AUTHORS", vec2(screen_width() / 2.0 - button_width / 2.0, button_y + button_spacing * 4.0), self.button_textures.clone()),
            Button::new("EXIT", vec2(screen_width() / 2.0 - button_width / 2.0, button_y + button_spacing * 5.0), self.button_textures.clone()),
        ];

    }

}