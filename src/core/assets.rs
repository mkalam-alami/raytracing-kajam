use png::Transformations;
use std::fs::File;

use super::colors::Color;

#[derive(Clone)]
pub struct ImageMetadata {
  pub width: i32,
  pub height: i32,
  pub alpha: bool,
  pub bytes_per_pixel: u8
}

#[derive(Clone)]
pub struct Image {
  pub meta: ImageMetadata,
  pub bytes: Vec<u8>,
}

impl Image {
  pub fn get(&self, i: i32, j: i32, out: &mut Color) {
    let start_index = ((j * self.meta.width + i) * self.meta.bytes_per_pixel as i32) as usize;
    out.copy_from_slice(&self.bytes[start_index..start_index + 4]);
  }
}

pub fn load_png(path: &str) -> Image {
  let mut full_path = String::new();
  full_path.push_str("src/assets/");
  full_path.push_str(path);

  let mut build_full_path = String::new();
  build_full_path.push_str("assets/");
  build_full_path.push_str(path);

  // println!("{}", current_dir().unwrap().to_str().unwrap());
  // println!("{}", full_path);

  let mut decoder = png::Decoder::new(File::open(full_path).or_else(|_| File::open(build_full_path)).unwrap());
  decoder.set_transformations(Transformations::EXPAND);
  let (meta, mut reader) = decoder.read_info().unwrap();
  let mut bytes = vec![0; meta.buffer_size()];
  reader.next_frame(&mut bytes).unwrap();

  let mut alpha = true;
  if meta.color_type == png::ColorType::RGB {
    bytes = rgb_to_rgba(bytes);
    alpha = false;
  }

  // println!(
  //   "{} {} {} {}\n{} {} {} {}",
  //   bytes[0], bytes[1], bytes[2], bytes[3],
  //   bytes[4], bytes[5], bytes[6], bytes[7]
  // );
  Image {
    meta: ImageMetadata {
      width: meta.width as i32,
      height: meta.height as i32,
      alpha,
      bytes_per_pixel: 4
    },
    bytes,
  }
}

fn rgb_to_rgba(rgb: Vec<u8>) -> Vec<u8> {
  rgb
    .chunks(3)
    .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 255].to_vec())
    .collect::<Vec<u8>>()
}
