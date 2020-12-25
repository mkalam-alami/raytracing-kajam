use crate::moving_box::MovingBox;
use crate::core::draw::fill;

#[derive(Clone)]
pub struct MainScene {
    moving_box: MovingBox
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            moving_box: MovingBox::new()
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {
        self.moving_box.update();
    }

    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8]) {
        fill(frame, &[0, 0, 0, 0]);
        self.moving_box.draw(frame);
    }

}
