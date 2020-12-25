use super::{config, math::clamp};

const PIXEL_BYTES: usize = 4;

pub fn fill_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: &[u8; 4]) {
    for row in clamp(y, 0, config::HEIGHT)..clamp(y + height, 0, config::HEIGHT) {
        let frame_row_index = row * config::WIDTH;
        let x_start = clamp(x, 0, config::WIDTH - 1);
        let x_end = clamp(x + width, 0, config::WIDTH - 1);
        fill_chunk(
            frame,
            (frame_row_index + x_start) as usize,
            (frame_row_index + x_end) as usize,
            color,
        );
    }
}

#[allow(dead_code)]
pub fn draw_pixel(frame: &mut [u8], x: i32, y: i32, color: &[u8; 4]) {
    let pixel_index = x + y * config::WIDTH;
    if pixel_index > 0 {
        fill_chunk(frame, pixel_index as usize, pixel_index as usize, color);
    }
}

pub fn fill_chunk(frame: &mut [u8], pixel_start: usize, pixel_end: usize, color: &[u8; 4]) {
    let frame_start = pixel_start * PIXEL_BYTES;
    let frame_end = pixel_end * PIXEL_BYTES;

    if frame_end >= frame_start && frame_end < frame.len() {
        for pixel in frame[frame_start..frame_end].chunks_exact_mut(PIXEL_BYTES) {
            pixel.copy_from_slice(color);
        }
    }
}

pub fn fill(frame: &mut [u8], color: &[u8; 4]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(color);
    }
}
