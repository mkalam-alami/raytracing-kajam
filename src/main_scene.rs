use crate::{core::{colors::Color, game::GameState}, palette::Palette, point::Point};
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
        let palette = Palette::new("palette.png");
        let map = Map::new("map.png", palette.clone());
        let mut map_preview = Entity::new(EntityShape::IMAGE("map.png".to_string()));
        map_preview.pos += Point::new(10., 10.);
        let mut ceiling_color: Color = Default::default();

        palette.pick(Palette::ceiling_color_id() as usize, &mut ceiling_color);

        // XXX Use references over clones
        Self {
            ceiling_color,
            map_preview,
            raycaster: Raycaster::new(map.clone(), palette),
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
