use super::{config, math::clamp};

const PIXEL_BYTES: usize = 4;

pub fn draw_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: &[u8; 4]) {
    for row in y..y + height {
        let frame_row_index = row * config::WIDTH;
        let x_start = clamp(x, 0, config::WIDTH - 1);
        let x_end = clamp(x + width, 0, config::WIDTH - 1);
        if frame_row_index + x_start > 0 && frame_row_index + x_end < frame.len() as i32 {
            draw_chunk(frame, (frame_row_index + x_start) as usize, (frame_row_index + x_end) as usize, color);
        }
    }
}

#[allow(dead_code)]
pub fn draw_pixel(frame: &mut [u8], x: i32, y: i32, color: &[u8; 4]) {
    let pixel_index = x + y * config::WIDTH;
    if pixel_index > 0 && pixel_index < frame.len() as i32 {
        draw_chunk(frame, pixel_index as usize, 1, color);
    }
}

pub fn draw_chunk(frame: &mut [u8], pixel_start: usize, pixel_end: usize, color: &[u8; 4]) {
    for pixel in pixel_start..pixel_end {
        let frame_index = pixel * PIXEL_BYTES;
        if frame_index + PIXEL_BYTES < frame.len() {
            frame[frame_index..frame_index + PIXEL_BYTES].copy_from_slice(color);
        }
    }
}
