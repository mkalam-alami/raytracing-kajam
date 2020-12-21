use amethyst::{GameData, SimpleState, StateData};

use crate::{camera::init_camera, paddle::init_paddles};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);
        init_paddles(world);
    }
}
