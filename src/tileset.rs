
use core::panic;

use crate::core::{assets::{Image, load_png}, colors::Color};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Tileset {
  palette: Vec<[u8; 4]>,
  textures: Vec<Image>
}

#[allow(dead_code)]
impl Tileset {
  pub fn new(path: &str) -> Self {
    let image = load_png(path);
    Self {
      palette: image.bytes.chunks(4)
        .map(|chunk| {
          // println!("{}, {}, {}, {}", chunk[0], chunk[1], chunk[2], chunk[3]);
          [chunk[0], chunk[1], chunk[2], chunk[3]]
        })
        .collect(),
      textures: [
        load_png("door_0.png"),
        load_png("door_1.png"),
        load_png("door_2.png"),
        load_png("door_3.png"),
        load_png("door_4.png"),
        load_png("door_5.png"),
        load_png("door_6.png"),
        load_png("door_7.png"),
        load_png("door_8.png"),
        load_png("door_9.png"),

        load_png("floor_m7.png"),
        load_png("floor_p1.png"),
        load_png("floor_p2.png"),
        load_png("floor_p3.png"),
        load_png("floor_m1.png"),
        load_png("floor_m2.png"),
        load_png("floor_m3.png"),

        load_png("you_win.png") // referenced as n - 1
      ].to_vec()
    }
  }

  pub fn pick(&self, color_id: usize, out: &mut Color) {
    out.copy_from_slice(&self.palette[color_id]);
  }

  pub fn get_color_id(&self, color: &[u8]) -> u8 {
    self.palette.iter()
      .enumerate()
      .find(|(_index, tileset_color)| tileset_color[0] == color[0] && tileset_color[1] == color[1] && tileset_color[2] == color[2])
      .map(|(index, _tileset_color)| index as u8)
      .expect(format!("unknown color id: # {}{}{}", format!("{:x}", color[0]), format!("{:x}", color[1]), format!("{:x}", color[2])).as_str())
  }

  pub fn ceiling_color_id() -> u8 {
    18
  }

  pub fn default_floor_color_id() -> u8 {
    0
  }

  pub fn default_wall_color_id() -> u8 {
    10
  }

  pub fn get_size(&self) -> u8 {
    self.palette.len() as u8
  }

  pub fn is_rendered_as_floor(color_id: u8) -> bool {
    color_id < 10
  }

  pub fn is_rendered_as_wall(color_id: u8) -> bool {
    color_id >= 10 && color_id != 20 /* door 0 */
  }

  pub fn is_colliding(color_id: u8) -> bool {
    color_id >= 10 && color_id != 20 /* door 0 */
  }

  pub fn is_spawn(color_id: u8) -> bool {
    color_id == 9
  }

  pub fn is_spawn_dir(color_id: u8) -> bool {
    color_id == 8
  }

  pub fn is_textured(color_id: u8) -> bool {
    color_id >= 19 || (color_id > 0 && color_id < 8)
  }

  pub fn get_texture(&self, color_id: u8) -> &Image {
    if color_id >= 20 {
      self.textures.get((color_id - 20) as usize).unwrap()
    } else if color_id == 19 {
      self.textures.get(self.textures.len() - 1).unwrap()
    } else if color_id < 8 {
      self.textures.get((color_id + 9) as usize).unwrap()
    } else {
      panic!("no texture for color_id {}", color_id);
    }
  }

  pub fn is_trigger(color_id: u8) -> bool {
    color_id >= 1 && color_id < 8
  }

  pub fn get_door_color_id(door_value: u8) -> u8 {
    door_value + 20
  }

  pub fn get_door_value(color_id: u8) -> Option<u8> {
    if color_id >= 20 {
      Some(color_id - 20)
    } else {
      None
    }
  }

}
