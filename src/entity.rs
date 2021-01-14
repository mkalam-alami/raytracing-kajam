use std::ops::Deref;

use crate::{core::{assets::{Image, load_png}, draw::{draw_image, draw_rect, fill_circle, fill_rect}}, point::Point};

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
    pub pos: Point,
    pub velocity: Point,
    pub color: [u8;4],
    pub size: f32,
    shape: EntityShape,
    image: Option<Box<Image>>
}

#[allow(dead_code)]
impl Entity {
    pub fn new(shape: EntityShape) -> Self {
        Self {
            shape: shape.clone(),
            image: match shape {
                EntityShape::IMAGE(ref path) => Some(Box::new(load_png(path))),
                _ => None
            },
            pos: Point::new(0., 0.),
            velocity: Point::new(0., 0.),
            size: 64.,
            color: [255, 255, 255, 255],
        }
    }

    pub fn update(&mut self) {
        // if self.pos.x <= PADDING || self.pos.x + SIZE > config::SCREEN_WIDTH as f32 - PADDING {
        //     self.velocity.x *= -1.0;
        // }
        // if self.pos.y <= PADDING || self.pos.y + SIZE > config::SCREEN_HEIGHT as f32 - PADDING {
        //     self.velocity.y *= -1.0;
        // }
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        match self.shape {
           EntityShape::BOX => fill_rect(frame, self.pos.x as i32, self.pos.y as i32, self.size as i32, self.size as i32, &self.color),
           EntityShape::BOXWIREFRAME => draw_rect(frame, self.pos.x as i32, self.pos.y as i32, self.size as i32, self.size as i32, &self.color),
           EntityShape::CIRCLE => fill_circle(frame, (self.pos.x + self.size / 2.0) as i32, (self.pos.y + self.size / 2.0) as i32, (self.size as f32 * 0.5) as i32, &self.color),
           EntityShape::IMAGE(_) => {
               if self.image.is_some() {
                   draw_image(frame, self.pos.x as i32, self.pos.y as i32, &self.image.as_ref().unwrap().deref())
               }
           }
        }
    }
}
