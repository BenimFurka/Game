

use game::gamestate::GameState;
use macroquad::prelude::*;
use menu::menu::{Menu, MenuAction};
use utils::{generate_seed::generate_seed, mod_loader::ModLoader, system::SystemInfo};

mod save;
mod utils;
mod menu;
mod game;
fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        fullscreen: false,
        platform: {
            #[cfg(target_os = "linux")]
            {
                miniquad::conf::Platform {
                    linux_backend: miniquad::conf::LinuxBackend::X11,
                    linux_x11_gl: true,
                    swap_interval: Some(1),
                    ..Default::default()
                }
            }
            #[cfg(target_os = "macos")]
            {
                miniquad::conf::Platform {
                    apple_gfx_api: miniquad::conf::AppleGfxApi::Metal,
                    swap_interval: Some(1),
                    ..Default::default()
                }
            }
            #[cfg(target_os = "windows")]
            {
                miniquad::conf::Platform {
                    swap_interval: Some(1),
                    ..Default::default()
                }
            }
        },
        ..Default::default()
    }
}
    
#[macroquad::main(window_conf)]
async fn main() {

    let mut game_state = None;
    let mut menu = Menu::new().await;
    let mut system_info = SystemInfo::new();
    let mut loader = ModLoader::new();
    let _ = loader.load_mods();
    menu.init().await;

    loop {
        let delta_time = get_frame_time();
        system_info.update();

        match &mut game_state {
            None => {
                match menu.update() {
                    MenuAction::NewGame => {
                        let biomes = loader.get_biomes().to_vec();
                        let tiles = loader.get_tiles().to_vec();
                        
                        let seed = generate_seed();
                        game_state = Some(GameState::new(seed, biomes, tiles).await);
                    }
                    MenuAction::Exit => break,
                    MenuAction::None => {}
                    _ => {}
                }
                menu.draw();
            }
            Some(state) => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = None;
                } else {
                    state.update(delta_time, &system_info).await;
                }
            }
        }


        next_frame().await;
    }
}

