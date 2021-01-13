use png::Transformations;
use std::fs::File;

#[derive(Clone)]
pub struct ImageMetadata {
  pub width: i32,
  pub height: i32,
  pub alpha: bool
}

#[derive(Clone)]
pub struct Image {
  pub meta: ImageMetadata,
  pub bytes: Vec<u8>,
}

pub fn load_png(path: &str) -> Image {
  let mut full_path = String::new();
  full_path.push_str("src/assets/"); // TODO Better handling of dev vs. build
  full_path.push_str(path);

  // println!("{}", current_dir().unwrap().to_str().unwrap());
  // println!("{}", full_path);

  let mut decoder = png::Decoder::new(File::open(full_path).unwrap());
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
      alpha
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
