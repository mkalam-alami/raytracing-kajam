use crate::moving_box::MovingBox;

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
        MainScene::clear_screen(frame);
        self.moving_box.draw(frame);
    }

    fn clear_screen(frame: &mut [u8]) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 0]);
        }
    }
}
