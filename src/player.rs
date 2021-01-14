use crate::point::Point;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const MOV_SPEED: f32 = 0.03;
const ROT_SPEED: f32 = 1.6;

#[derive(Clone)]
pub struct Player {
  pub pos: Point,
  pub dir: Point
}

impl Player {
  pub fn new(pos: Point, dir: Point) -> Self {
    Self {
      pos: pos.clone(),
      dir: dir.clone()
    }
  }

  pub fn get_camera_plane(&self) -> Point {
    self.dir.rotate(90.) * 0.66
  }

  pub fn update(&mut self, input: &WinitInputHelper) {
      // XXX Use dir vector
      if input.key_held(VirtualKeyCode::Left) {
        self.dir = self.dir.rotate(-ROT_SPEED);
      }
      if input.key_held(VirtualKeyCode::Right) {
        self.dir = self.dir.rotate(ROT_SPEED);
      }
      if input.key_held(VirtualKeyCode::Up) {
        self.pos += self.dir * MOV_SPEED;
      }
      if input.key_held(VirtualKeyCode::Down) {
        self.pos -= self.dir * MOV_SPEED;
      }
  }
}
