use crate::raycaster::Raycaster;
use crate::{core::colors, entity::{Entity, EntityShape}};
use crate::core::draw::fill;
use crate::map::Map;

#[derive(Clone)]
pub struct MainScene {
    entities: Vec<Entity>,
    raycaster: Raycaster
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            entities: [
                Entity::new(EntityShape::BOX, colors::COLOR_PURPLE),
                Entity::new(EntityShape::BOXWIREFRAME, colors::COLOR_WHITE),
                Entity::new(EntityShape::CIRCLE, colors::COLOR_YELLOW),
                Entity::new(EntityShape::IMAGE("alakajam.png".to_string()), colors::COLOR_WHITE)
            ].to_vec(),
            raycaster: Raycaster::new(Map::new())
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {
        self.entities.iter_mut()
            .for_each(|e| e.update());
    }

    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8]) {
        fill(frame, &[0, 0, 0, 0]);
        self.entities.iter()
            .for_each(|e| e.draw(frame));
        self.raycaster.draw(frame);
    }

}
