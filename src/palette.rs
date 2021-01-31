
use crate::core::assets::{Image, load_png};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Palette {
  colors: Vec<[u8; 4]>,
  pub textures: Vec<Image>
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
        load_png("map.png")
      ].to_vec()
    }
  }

  pub fn pick(&self, color_id: usize) -> &[u8; 4] {
    &self.colors[color_id]
  }

  pub fn get_color_id(&self, color: &[u8]) -> u8 {
    self.colors.iter()
      .enumerate()
      .find(|(_index, palette_color)| palette_color[0] == color[0] && palette_color[1] == color[1] && palette_color[2] == color[2])
      .map(|(index, _palette_color)| index as u8)
      .expect(format!("unknown color id: {} {} {}", color[0], color[1], color[2]).as_str())
  }

  pub fn get_size(&self) -> u8 {
    self.colors.len() as u8
  }
}
