use crate::point::Point;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

#[derive(Clone)]
pub struct Player {
  pub pos: Point,
  pub dir: Point
}

impl Player {
  pub fn new() -> Self {
    Self {
    pos: Point { x: 22., y: 22. },
    dir: Point { x: -1., y: 0. }
    }
  }

  pub fn update(&mut self, input: &WinitInputHelper) {
      let mut d_pos = Point::new(0., 0.);
      let speed = 0.03;

      // XXX Use dir vector
      if input.key_held(VirtualKeyCode::Left) { d_pos.y -= speed; }
      if input.key_held(VirtualKeyCode::Right) { d_pos.y += speed; }
      if input.key_held(VirtualKeyCode::Up) { d_pos.x -= speed; }
      if input.key_held(VirtualKeyCode::Down) { d_pos.x += speed; }
      self.pos = self.pos + d_pos;
  }
}
