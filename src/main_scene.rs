use crate::player::Player;
use crate::raycaster::Raycaster;
use crate::{core::colors, entity::Entity};
use crate::core::draw::fill;
use crate::entity::EntityShape;
use crate::map::Map;
use winit_input_helper::WinitInputHelper;

#[derive(Clone)]
pub struct MainScene {
    entities: Vec<Entity>,
    raycaster: Raycaster,
    player: Player
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            entities: [
                Entity::new(EntityShape::BOX, colors::COLOR_PURPLE),
                Entity::new(EntityShape::BOXWIREFRAME, colors::COLOR_WHITE),
                Entity::new(EntityShape::CIRCLE, colors::COLOR_YELLOW),
                Entity::new(EntityShape::IMAGE("alakajam.png".to_string()), colors::COLOR_WHITE),
                Entity::new(EntityShape::IMAGE("jellymancer.png".to_string()), colors::COLOR_WHITE)
            ].to_vec(),
            raycaster: Raycaster::new(Map::new()),
            player: Player::new()
        }
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.entities.iter_mut()
            .for_each(|e| e.update());
        self.player.update(input);
    }

    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8]) {
        fill(frame, &colors::COLOR_DARK_BLUE);
        self.raycaster.draw(frame, &self.player);
        self.entities.iter()
            .for_each(|e| e.draw(frame));
    }

}
