
#[derive(Clone, Copy)]
pub struct Point {
  pub x: f32,
  pub y: f32
}

impl Point {
  pub fn new(x: f32, y: f32) -> Point {
    Point { x, y }
  }
  pub fn floor(&self) -> Point {
    Point { x: self.x.floor(), y: self.y.floor() }
  }
}

impl std::ops::Add for Point {
  type Output = Point;
  fn add(self, other: Point) -> <Self as std::ops::Add<Point>>::Output {
    Point { x: self.x + other.x, y: self.y + other.y }
  }
}
impl std::ops::Mul<Point> for Point {
  type Output = Point;
  fn mul(self, other: Point) -> Point {
    Point { x: self.x * other.x, y: self.y * other.y }
  }
}
impl std::ops::Mul<f32> for Point {
  type Output = Point;
  fn mul(self, other: f32) -> Point {
    Point { x: self.x * other, y: self.y * other }
  }
}