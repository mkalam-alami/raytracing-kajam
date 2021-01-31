use crate::{palette::Palette, point::Point};
use crate::player::Player;
use crate::raycaster::Raycaster;
use crate::{core::colors, entity::Entity};
use crate::core::draw::fill;
use crate::entity::EntityShape;
use crate::map::Map;
use winit_input_helper::WinitInputHelper;

#[derive(Clone)]
pub struct MainScene {
    map_preview: Entity,
    raycaster: Raycaster,
    player: Player
}

impl MainScene {
    pub fn new() -> Self {
        let palette = Palette::new("palette.png");
        let map = Map::new("map.png", &palette);
        let mut map_preview = Entity::new(EntityShape::IMAGE("map.png".to_string()));
        map_preview.pos += Point::new(10., 10.);

        // XXX Use references over clones
        Self {
            map_preview,
            raycaster: Raycaster::new(map.clone(), palette.clone()),
            player: Player::new(map.spawn_pos.clone(), map.spawn_dir.clone(), Box::new(map))
        }
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.player.update(input);
    }

    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8]) {
        fill(frame, &colors::COLOR_DARK_BLUE);
        self.raycaster.draw(frame, &self.player);
        self.map_preview.draw(frame);
    }

}
