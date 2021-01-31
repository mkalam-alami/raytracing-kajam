use crate::{core::{colors::Color, game::GameState}, tileset::Tileset, point::Point};
use crate::player::Player;
use crate::raycaster::Raycaster;
use crate::{entity::Entity};
use crate::core::draw::fill;
use crate::entity::EntityShape;
use crate::map::Map;
use winit_input_helper::WinitInputHelper;

#[derive(Clone)]
pub struct MainScene {
    map_preview: Entity,
    raycaster: Raycaster,
    player: Player,
    ceiling_color: Color
}

impl MainScene {
    pub fn new() -> Self {
        let tileset = Tileset::new("palette.png");
        let map = Map::new("map.png", tileset.clone());
        let mut map_preview = Entity::new(EntityShape::IMAGE("map.png".to_string()));
        map_preview.pos += Point::new(10., 10.);
        let mut ceiling_color: Color = Default::default();

        tileset.pick(Tileset::ceiling_color_id() as usize, &mut ceiling_color);

        // XXX Use references over clones
        Self {
            ceiling_color,
            map_preview,
            raycaster: Raycaster::new(map.clone(), tileset),
            player: Player::new(map.spawn_pos.clone(), map.spawn_dir.clone(), Box::new(map))
        }
    }

    pub fn update(&mut self, input: &WinitInputHelper, game_state: &GameState) {
        self.player.update(input, game_state);
    }

    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8], game_state: &GameState) {
        fill(frame, &self.ceiling_color);
        self.raycaster.draw(frame, &self.player, game_state);
        self.map_preview.draw(frame);
    }

}
