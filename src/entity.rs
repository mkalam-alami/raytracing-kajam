use std::ops::Deref;

use crate::core::{assets::{Image, load_png}, config::{self, Color}, draw::{draw_image, draw_rect, fill_circle, fill_rect}};

pub const SIZE: f32 = 64.0;
pub const PADDING: f32 = -30.0;

#[derive(Clone)]
#[allow(dead_code)]
pub enum EntityShape {
    BOX,
    BOXWIREFRAME,
    CIRCLE,
    IMAGE(String)
}

#[derive(Clone)]
pub struct Entity {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    shape: EntityShape,
    color: [u8;4],
    image: Option<Box<Image>>
}

#[allow(dead_code)]
impl Entity {
    pub fn new(shape: EntityShape, color: Color) -> Self {
        Self {
            x: rand::random::<f32>() * config::SCREEN_WIDTH as f32 / 2. + config::SCREEN_WIDTH as f32 / 4.,
            y: rand::random::<f32>() * config::SCREEN_HEIGHT as f32 / 2. + config::SCREEN_WIDTH as f32 / 4.,
            velocity_x: 0.5 * (if rand::random::<bool>() { 1.0 } else { -1.0 }),
            velocity_y: 0.5 * (if rand::random::<bool>() { 1.0 } else { -1.0 }),
            shape: shape.clone(),
            color,
            image: match shape {
                EntityShape::IMAGE(ref path) => Some(Box::new(load_png(path))),
                _ => None
            }
        }
    }

    pub fn update(&mut self) {
        if self.x <= PADDING || self.x + SIZE > config::SCREEN_WIDTH as f32 - PADDING {
            self.velocity_x *= -1.0;
        }
        if self.y <= PADDING || self.y + SIZE > config::SCREEN_HEIGHT as f32 - PADDING {
            self.velocity_y *= -1.0;
        }

        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        match self.shape {
           EntityShape::BOX => fill_rect(frame, self.x as i32, self.y as i32, SIZE as i32, SIZE as i32, &self.color),
           EntityShape::BOXWIREFRAME => draw_rect(frame, self.x as i32, self.y as i32, SIZE as i32, SIZE as i32, &self.color),
           EntityShape::CIRCLE => fill_circle(frame, (self.x + SIZE / 2.0) as i32, (self.y + SIZE / 2.0) as i32, (SIZE as f32 * 0.5) as i32, &self.color),
           EntityShape::IMAGE(_) => {
               if self.image.is_some() {
                   // TODO
                   draw_image(frame, self.x as i32, self.y as i32, &self.image.as_ref().unwrap().deref())
               }
           }
        }
    }
}
