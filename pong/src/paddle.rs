use amethyst::{core::Transform, ecs::{Component, DenseVecStorage}, prelude::*, shred::World};

use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};

const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;

#[derive(Debug, Clone, Copy)]
pub enum Side {
  Left,
  Right
}

pub struct Paddle {
  pub side: Side
}

impl Paddle {
  fn new(side: Side) -> Paddle {
      Paddle { side }
  }
}
impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_paddles(world: &mut World) {
  world.register::<Paddle>(); // only needed because there are no systems yet

  for side in [Side::Left, Side::Right].iter() {
    let direction: f32 = match side {
      Side::Left => -1.,
      Side::Right => 1.
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(
      (ARENA_WIDTH - PADDLE_WIDTH * 1.5) * direction,
      ARENA_HEIGHT / 2.,
      0.);

    world.create_entity()
      .with(transform)
      .with(Paddle::new(*side))
      .build();
  }
}