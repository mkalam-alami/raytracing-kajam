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
    map: Map,
    map_preview: Entity,
    raycaster: Raycaster,
    player: Player,
    ceiling_color: Color
}

impl MainScene {
    pub fn new() -> Self {
        let tileset = Tileset::new("palette.png");
        let map = Map::new("map.png", tileset.clone());
        let player = Player::new(map.spawn_pos, map.spawn_dir);
        let mut map_preview = Entity::new(EntityShape::IMAGE("map.png".to_string()));
        map_preview.pos += Point::new(10., 10.);
        let mut ceiling_color: Color = Default::default();

        tileset.pick(Tileset::ceiling_color_id() as usize, &mut ceiling_color);

        Self {
            ceiling_color,
            map_preview,
            raycaster: Raycaster::new(tileset),
            player,
            map
        }
    }

    pub fn update(&mut self, input: &WinitInputHelper, game_state: &GameState) {
        self.player.update(&self.map, input, game_state);
        if self.player.is_triggering_this_update {
            self.map.trigger(self.player.get_current_cell());
        }
    }

    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8], game_state: &GameState) {
        fill(frame, &self.ceiling_color);
        self.raycaster.draw(&self.map, frame, &self.player, game_state);
        // self.map_preview.draw(frame);
    }

}
