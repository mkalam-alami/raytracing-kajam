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
        self.moving_box.draw(frame);

        // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        //     let x = (i % constants::WIDTH as usize) as i16;
        //     let y = (i / constants::WIDTH as usize) as i16;

        //     let inside_the_box = x >= self.box_x
        //         && x < self.box_x + constants::BOX_SIZE
        //         && y >= self.box_y
        //         && y < self.box_y + constants::BOX_SIZE;

        //     let rgba = if inside_the_box {
        //         [0x5e, 0x48, 0xe8, 0xff]
        //     } else {
        //         [0x48, 0xb2, 0xe8, 0xff]
        //     };

        //     pixel.copy_from_slice(&rgba);
        // }
    }
}
