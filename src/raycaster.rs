use crate::map::Map;
use crate::point::Point;
use crate::core::config;

#[derive(Clone)]
pub struct Raycaster {
  map: Map,
  position: Point,
  direction: Point,
  camera_plane: Point
}

#[allow(dead_code)]
impl Raycaster {
  pub fn new(map: Map) -> Self {
    Self {
      map,
      position: Point { x: 22., y: 22. },
      direction: Point { x: -1., y: 0. },
      camera_plane: Point { x: 0., y: 0.66 }
    }
  }

  pub fn draw(&self, _frame: &mut [u8]) {
    for x in 0..config::SCREEN_WIDTH {
      let camera_x = 2. * x as f32 / config::SCREEN_WIDTH as f32 - 1.; // [-1;1]
      let _ray_dir = self.direction + self.camera_plane * camera_x;
      // TODO
    }
  }

}