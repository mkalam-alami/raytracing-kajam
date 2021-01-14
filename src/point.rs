use std::ops;
use std::fmt;


#[derive(Clone, Copy)]
pub struct Point {
  pub x: f32,
  pub y: f32
}

#[allow(dead_code)]
impl Point {
  pub fn new(x: f32, y: f32) -> Point {
    Point { x, y }
  }
  pub fn floor(&self) -> Point {
    Point { x: self.x.floor(), y: self.y.floor() }
  }
  pub fn get_magnitude(&self) -> f32 {
    (self.x.abs() * self.x.abs() + self.y.abs() * self.y.abs()).sqrt()
  }
  pub fn normalize(&self) -> Point {
    let magnitude = self.get_magnitude();
    Point { x: self.x / magnitude, y: self.y / magnitude }
  }
  pub fn rotate(&self, degrees: f32) -> Point {
    let rads = degrees * 0.01745329252;
    Point {
      x: self.x * rads.cos() - self.y * rads.sin(),
      y: self.x * rads.sin() + self.y * rads.cos()
    }
  }
}

// NOTE Probably a way to derive them in some way
// Also what about implementing these for multiple right-hand types?

impl ops::Add for Point {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point { x: self.x + other.x, y: self.y + other.y }
  }
}
impl ops::AddAssign for Point {
  fn add_assign(&mut self, other: Point) {
      self.x += other.x;
      self.y += other.y;
  }
}
impl ops::Sub for Point {
  type Output = Point;
  fn sub(self, other: Point) -> Point {
    Point { x: self.x - other.x, y: self.y - other.y }
  }
}
impl ops::SubAssign for Point {
  fn sub_assign(&mut self, other: Point) {
      self.x -= other.x;
      self.y -= other.y;
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