use std::usize;

use crate::{core::assets::Image, point::Point, trigger::trigger};
use crate::core::assets::load_png;
use crate::tileset::Tileset;

#[derive(Clone)]
pub struct Map {
  tiles: Vec<Vec<u8>>,
  pub spawn_pos: Point,
  pub spawn_dir: Point,
  pub tileset: Box<Tileset>,
  pub doors: Vec<(usize, usize)>
}

#[allow(dead_code)]
impl Map {
  pub fn new(path: &str, tileset: Tileset) -> Self {
    let image = load_png(path);
    Map::parse_tiles(image, tileset)
  }

  pub fn trigger(&mut self, tile: (usize, usize)) {
    let color_id = *self.get(tile.0, tile.1).unwrap();
    if Tileset::is_trigger(color_id) {
      let mut nearest_door = *self.doors.get(0).unwrap();
      for door in self.doors.clone() {
        if Map::tile_distance(door, tile) < Map::tile_distance(nearest_door, tile) {
          nearest_door = door;
        }
      }
      trigger(self, &nearest_door, color_id);
    }
  }

  fn tile_distance(tile1: (usize, usize), tile2: (usize, usize)) -> usize {
    (Point::new(tile1.0 as f32, tile1.1 as f32) - Point::new(tile2.0 as f32, tile2.1 as f32)).get_magnitude() as usize
  }

  fn parse_tiles(image: Image, tileset: Tileset) -> Self {
    let mut spawn_pos = Point::new(0., 0.);
    let mut spawn_look_at_pos = Point::new(1., 0.);
    let mut doors: Vec<(usize, usize)> = Vec::new();

    let tiles = image.bytes.chunks(image.meta.width as usize * image.meta.bytes_per_pixel as usize)
      .enumerate()
      .map(|(y, row)| row.to_vec().chunks(image.meta.bytes_per_pixel as usize)
        .enumerate()
        .map(|(x, chunk)| {
          let mut color_id = tileset.get_color_id(&chunk);
          if Tileset::is_spawn(color_id) {
            spawn_pos = Point::new(x as f32, y as f32);
            color_id = Tileset::default_floor_color_id();
          } else if Tileset::is_spawn_dir(color_id) {
            spawn_look_at_pos = Point::new(x as f32, y as f32);
            color_id = Tileset::default_floor_color_id();
          } else if Tileset::get_door_value(color_id).is_some() {
            doors.push((x, y));
          }
          color_id
        })
        .collect::<Vec<u8>>())
      .collect::<Vec<Vec<u8>>>();

    Self {
      tiles,
      spawn_pos,
      spawn_dir: (spawn_look_at_pos - spawn_pos).normalize(),
      tileset: Box::new(tileset),
      doors
    }
  }

  pub fn get(&self, i: usize, j: usize) -> Option<&u8> {
    self.tiles.get(j).and_then(|row| row.get(i))
  }

  pub fn set(&mut self, i: usize, j: usize, color_id: u8) {
    let row = self.tiles.get_mut(j).unwrap();
    row[i] = color_id;
  }

  pub fn width(&self) {
    self.tiles.get(0).unwrap().len();
  }

  pub fn height(&self) {
    self.tiles.len();
  }
}
