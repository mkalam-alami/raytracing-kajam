use crate::core::config;

pub const SIZE: i16 = 64;

#[derive(Clone)]
pub struct MovingBox {
    x: i16,
    y: i16,
    velocity_x: i16,
    velocity_y: i16
}

impl MovingBox {
    pub fn new() -> Self {
        Self {
            x: 24,
            y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    pub fn update(&mut self) {
        if self.x <= 0 || self.x + SIZE > config::WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.y <= 0 || self.y + SIZE > config::HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        // TODO Only explore required pixels 
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % config::WIDTH as usize) as i16;
            let y = (i / config::WIDTH as usize) as i16;

            let inside_the_box = x >= self.x
                && x < self.x + SIZE
                && y >= self.y
                && y < self.y + SIZE;

            if inside_the_box {
                pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]);
            }
        }
    }
}
