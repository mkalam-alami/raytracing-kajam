use config::{COLOR_PURPLE, COLOR_YELLOW};

use crate::core::{config, draw::{draw_rect, fill_circle, fill_rect}};

pub const SIZE: i32 = 64;
pub const PADDING: i32 = -30;

#[derive(Clone)]
pub struct MovingBox {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32
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
        if self.x <= PADDING || self.x + SIZE > config::WIDTH - PADDING {
            self.velocity_x *= -1;
        }
        if self.y <= PADDING || self.y + SIZE > config::HEIGHT - PADDING {
            self.velocity_y *= -1;
        }

        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        fill_rect(frame, self.x, self.y, SIZE, SIZE, &COLOR_PURPLE);
        draw_rect(frame, self.x, self.y, SIZE, SIZE, &COLOR_YELLOW);
        fill_circle(frame, self.x + SIZE / 2, self.y + SIZE / 2, (SIZE as f32 * 0.4) as i32, &COLOR_YELLOW);
    }
}
