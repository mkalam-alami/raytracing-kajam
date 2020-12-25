use crate::core::{config::{self, Color}, draw::{draw_rect, fill_circle, fill_rect}};

pub const SIZE: i32 = 64;
pub const PADDING: i32 = -30;

#[derive(Clone)]
#[allow(dead_code)]
pub enum EntityShape {
    BOX,
    BOXWIREFRAME,
    CIRCLE
}

#[derive(Clone)]
pub struct Entity {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
    shape: EntityShape,
    color: [u8;4]
}

impl Entity {
    pub fn new(shape: EntityShape, color: Color) -> Self {
        Self {
            x: (rand::random::<f32>() * config::WIDTH as f32) as i32,
            y: (rand::random::<f32>() * config::HEIGHT as f32) as i32,
            velocity_x: 3 * (if rand::random::<bool>() { 1 } else { -1 }),
            velocity_y: 3 * (if rand::random::<bool>() { 1 } else { -1 }),
            shape,
            color
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
        match self.shape {
           EntityShape::BOX => fill_rect(frame, self.x, self.y, SIZE, SIZE, &self.color),
           EntityShape::BOXWIREFRAME => draw_rect(frame, self.x, self.y, SIZE, SIZE, &self.color),
           EntityShape::CIRCLE => fill_circle(frame, self.x + SIZE / 2, self.y + SIZE / 2, (SIZE as f32 * 0.5) as i32, &self.color)
        }
    }
}
