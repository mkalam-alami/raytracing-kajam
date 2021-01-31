
use crate::core::{assets::{Image, load_png}, colors::Color};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Palette {
  colors: Vec<[u8; 4]>,
  textures: Vec<Image>
}

#[allow(dead_code)]
impl Palette {
  pub fn new(path: &str) -> Self {
    let image = load_png(path);
    Self {
      colors: image.bytes.chunks(4)
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
        load_png("door_9.png")
      ].to_vec()
    }
  }

  pub fn pick(&self, color_id: usize, out: &mut Color) {
    out.copy_from_slice(&self.colors[color_id]);
  }

  pub fn get_color_id(&self, color: &[u8]) -> u8 {
    self.colors.iter()
      .enumerate()
      .find(|(_index, palette_color)| palette_color[0] == color[0] && palette_color[1] == color[1] && palette_color[2] == color[2])
      .map(|(index, _palette_color)| index as u8)
      .expect(format!("unknown color id: # {}{}{}", format!("{:x}", color[0]), format!("{:x}", color[1]), format!("{:x}", color[2])).as_str())
  }

  pub fn ceiling_color_id() -> u8 {
    10
  }

  pub fn default_floor_color_id() -> u8 {
    1
  }

  pub fn default_wall_color_id() -> u8 {
    10
  }

  pub fn get_size(&self) -> u8 {
    self.colors.len() as u8
  }

  pub fn is_rendered_as_floor(color_id: i8) -> bool {
    color_id < 10
  }

  pub fn is_rendered_as_wall(color_id: i8) -> bool {
    color_id >= 10
  }

  pub fn is_colliding(color_id: i8) -> bool {
    color_id >= 10 && color_id != 20 /* door 0 */
  }

  pub fn is_spawn(color_id: i8) -> bool {
    color_id == 9
  }

  pub fn is_spawn_dir(color_id: i8) -> bool {
    color_id == 8
  }

  pub fn is_textured(color_id: i8) -> bool {
    color_id >= 20
  }

  pub fn get_texture(&self, color_id: i8) -> &Image {
    self.textures.get((color_id - 20) as usize).unwrap()
  }
}
