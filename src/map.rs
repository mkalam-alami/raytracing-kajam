use crate::{core::assets::Image, point::Point};
use crate::core::assets::load_png;
use crate::palette::Palette;

#[derive(Clone)]
pub struct Map {
  tiles: Vec<Vec<u8>>,
  pub spawn_pos: Point,
  pub spawn_dir: Point
}

#[allow(dead_code)]
impl Map {
  pub fn new(path: &str, palette: &Palette) -> Self {
    let image = load_png(path);
    Map::parse_tiles(image, palette)
  }

  fn parse_tiles(image: Image, palette: &Palette) -> Self {
    let palette_size = palette.get_size();
    let mut spawn_pos = Point::new(0., 0.);
    let mut spawn_look_at_pos = Point::new(1., 0.);

    let tiles = image.bytes.chunks(image.meta.width as usize * image.meta.bytes_per_pixel as usize)
      .enumerate()
      .map(|(y, row)| row.to_vec().chunks(image.meta.bytes_per_pixel as usize)
        .enumerate()
        .map(|(x, chunk)| {
          let mut color_id = palette.get_color_id(&chunk);
          if palette.get_color_id(&chunk) == palette_size - 1 {
            spawn_pos = Point::new(x as f32, y as f32);
            color_id = 0;
          } else if palette.get_color_id(&chunk) == palette_size - 2 {
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
      spawn_dir: (spawn_look_at_pos - spawn_pos).normalize()
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
