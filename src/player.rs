use crate::{map::Map, point::Point};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const MOV_SPEED: f32 = 0.03;
const ROT_SPEED: f32 = 1.9;
const PLAYER_SIZE: f32 = 0.4 / MOV_SPEED;

#[derive(Clone)]
pub struct Player {
  pub pos: Point,
  pub dir: Point,
  pub map: Box<Map>
}

impl Player {
  pub fn new(pos: Point, dir: Point, map: Box<Map>) -> Self {
    Self {
      pos: pos.clone(),
      dir: dir.clone(),
      map
    }
  }

  pub fn get_camera_plane(&self) -> Point {
    self.dir.rotate(90.) * 0.66
  }

  pub fn update(&mut self, input: &WinitInputHelper) {
      if input.key_held(VirtualKeyCode::Left) {
        self.dir = self.dir.rotate(-ROT_SPEED);
      }
      if input.key_held(VirtualKeyCode::Right) {
        self.dir = self.dir.rotate(ROT_SPEED);
      }

      let mut dpos = Point::new(0., 0.);
      if input.key_held(VirtualKeyCode::Up) {
        dpos = self.dir * MOV_SPEED;
      }
      if input.key_held(VirtualKeyCode::Down) {
        dpos = self.dir * -MOV_SPEED;
      }

      let dx_collision = self.map.as_ref().get((self.pos.x + (dpos.x * PLAYER_SIZE)) as usize, self.pos.y as usize);
      if Some(&0) == dx_collision {
        self.pos.x = self.pos.x + dpos.x;
      }
      let dy_collision = self.map.as_ref().get(self.pos.x as usize, (self.pos.y + (dpos.y * PLAYER_SIZE)) as usize);
      if Some(&0) == dy_collision {
        self.pos.y = self.pos.y + dpos.y;
      }
  }
}
