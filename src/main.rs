#![deny(clippy::all)]
#![forbid(unsafe_code)]

#[allow(dead_code)]
mod core;

mod main_scene;
mod moving_box;

use pixels::Error;
use crate::core::game::{Game, GameSettings};
use crate::core::config;
use main_scene::MainScene;

fn main() -> Result<(), Error> {
    env_logger::init();

    let game_settings = GameSettings {
        width: config::WIDTH as u32,
        height: config::HEIGHT as u32,
        title: "Raycasting Kajam".to_string(),
        scene: MainScene::new()
    };

    Game::run(&game_settings)
}
