use std::ops;
use std::fmt;


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

impl ops::Add for Point {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point { x: self.x + other.x, y: self.y + other.y }
  }
}
impl ops::Sub for Point {
  type Output = Point;
  fn sub(self, other: Point) -> Point {
    Point { x: self.x - other.x, y: self.y - other.y }
  }
}
impl ops::Mul<Point> for Point {
  type Output = Point;
  fn mul(self, other: Point) -> Point {
    Point { x: self.x * other.x, y: self.y * other.y }
  }
}
impl ops::Mul<f32> for Point {
  type Output = Point;
  fn mul(self, other: f32) -> Point {
    Point { x: self.x * other, y: self.y * other }
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "Point[x={},y={}]", self.x, self.y)
  }
}