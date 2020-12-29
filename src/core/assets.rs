use std::fs::File;

#[derive(Clone)]
pub struct ImageMetadata {
  pub width: i32,
  pub height: i32
}

#[derive(Clone)]
pub struct Image {
  pub meta: ImageMetadata,
  pub bytes: Vec<u8>
}

pub fn load_png(path: &String) -> Image {
  let mut full_path = String::new();
  full_path.push_str("src/assets/"); // TODO Better handling of dev vs. build
  full_path.push_str(path);

  // println!("{}", current_dir().unwrap().to_str().unwrap());
  // println!("{}", full_path);

  let decoder = png::Decoder::new(File::open(full_path).unwrap());
  let (meta, mut reader) = decoder.read_info().unwrap();
  let mut bytes = vec![0; meta.buffer_size()];
  reader.next_frame(&mut bytes).unwrap();

  Image {
    meta: ImageMetadata { width: meta.width as i32, height: meta.height as i32 },
    bytes
  }
}
