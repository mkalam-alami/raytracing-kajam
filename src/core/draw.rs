use super::{config, math::clamp};

const PIXEL_BYTES: usize = 4;

#[allow(dead_code)]
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
pub fn draw_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: &[u8; 4]) {
    draw_line(frame, x, y, x + width, y, color);
    draw_line(frame, x, y, x, y + height, color);
    draw_line(frame, x, y + height, x + width, y + height, color);
    draw_line(frame, x + width, y, x + width, y + height, color);
}

#[allow(dead_code)]
pub fn draw_pixel(frame: &mut [u8], x: i32, y: i32, color: &[u8; 4]) {
    if x == clamp(x, 0, config::WIDTH - 1) && y == clamp(y, 0, config::HEIGHT - 1) {
        let pixel_index = (x + y * config::WIDTH) as usize;
        fill_chunk(frame, pixel_index, pixel_index, color);
    }
}

#[allow(dead_code)]
pub fn draw_line(frame: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32, color: &[u8; 4]) {
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

    let dx = (x2-x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2-y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    
    let mut x = x1;
    let mut y = y1;
    let mut err = dx + dy;

    loop {
        draw_pixel(frame, x, y, color);
        if x == x2 && y == y2 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy
        }
    }
}

#[allow(dead_code)]
pub fn fill_circle(frame: &mut [u8], center_x: i32, center_y: i32, radius: i32, color: &[u8; 4]) {
    let radius_f32 = radius as f32;
    for x in center_x-radius..center_x+radius+1 {
        for y in center_y-radius..center_y+radius+1 {
            if (((y-center_y).abs().pow(2) + (x-center_x).abs().pow(2)) as f32).sqrt() < radius_f32 {
                draw_pixel(frame, x, y, color);
            }
        }
    }
}

pub fn fill_chunk(frame: &mut [u8], pixel_start: usize, pixel_end: usize, color: &[u8; 4]) {
    let frame_start = pixel_start * PIXEL_BYTES;
    let frame_end = (pixel_end + 1) * PIXEL_BYTES;

    if frame_end >= frame_start && frame_end < frame.len() {
        for pixel in frame[frame_start..frame_end + 1].chunks_exact_mut(PIXEL_BYTES) {
            pixel.copy_from_slice(color);
        }
    }
}

pub fn fill(frame: &mut [u8], color: &[u8; 4]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(color);
    }
}
