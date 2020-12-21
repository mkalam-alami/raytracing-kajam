use amethyst::{core::Transform, prelude::*, renderer::Camera, shred::World};

use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};

pub fn init_camera(world: &mut World) {
  // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
  let mut transform = Transform::default();
  transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

  world
      .create_entity()
      .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
      .with(transform)
      .build();
}
