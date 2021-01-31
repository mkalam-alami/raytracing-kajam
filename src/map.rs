use crate::{core::assets::Image, point::Point};
use crate::core::assets::load_png;
use crate::tileset::Tileset;

#[derive(Clone)]
pub struct Map {
  tiles: Vec<Vec<u8>>,
  pub spawn_pos: Point,
  pub spawn_dir: Point,
  pub tileset: Box<Tileset>
}

#[allow(dead_code)]
impl Map {
  pub fn new(path: &str, tileset: Tileset) -> Self {
    let image = load_png(path);
    Map::parse_tiles(image, tileset)
  }

  fn parse_tiles(image: Image, tileset: Tileset) -> Self {
    let mut spawn_pos = Point::new(0., 0.);
    let mut spawn_look_at_pos = Point::new(1., 0.);

    let tiles = image.bytes.chunks(image.meta.width as usize * image.meta.bytes_per_pixel as usize)
      .enumerate()
      .map(|(y, row)| row.to_vec().chunks(image.meta.bytes_per_pixel as usize)
        .enumerate()
        .map(|(x, chunk)| {
          let mut color_id = tileset.get_color_id(&chunk);
          if Tileset::is_spawn(tileset.get_color_id(&chunk) as i8) {
            spawn_pos = Point::new(x as f32, y as f32);
            color_id = 0;
          } else if Tileset::is_spawn_dir(tileset.get_color_id(&chunk) as i8) {
            spawn_look_at_pos = Point::new(x as f32, y as f32);
            color_id = 0;
          }
          color_id
        })
        .collect::<Vec<u8>>())
      .collect::<Vec<Vec<u8>>>();

    Self {
      tiles,
      spawn_pos,
      spawn_dir: (spawn_look_at_pos - spawn_pos).normalize(),
      tileset: Box::new(tileset)
    }
  }

  pub fn get(&self, i: usize, j: usize) -> Option<&u8> {
    self.tiles.get(j).and_then(|row| row.get(i))
  }

  pub fn width(&self) {
    self.tiles.get(0).unwrap().len();
  }

  pub fn height(&self) {
    self.tiles.len();
  }
}
