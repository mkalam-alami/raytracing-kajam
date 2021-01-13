use std::cmp::min;

use super::{assets::Image, colors::Color, config, math::clamp};

const PIXEL_BYTES: usize = 4;

/// Fills a rectangle.
pub fn fill_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: &Color) {
    for row in clamp(y, 0, config::SCREEN_HEIGHT)..clamp(y + height, 0, config::SCREEN_HEIGHT) + 1 {
        let frame_row_index = row * config::SCREEN_WIDTH;
        let x_start = clamp(x, 0, config::SCREEN_WIDTH - 1);
        let x_end = clamp(x + width, 0, config::SCREEN_WIDTH - 1);
        fill_chunk(
            frame,
            (frame_row_index + x_start) as usize,
            (frame_row_index + x_end) as usize,
            color,
        );
    }
}

/// Draws the borders of a rectangle.
/// Prefer using the slightly faster `fill_rect()` when possible.
pub fn draw_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: &Color) {
    if width > 0 && y == clamp(y, 0, config::SCREEN_HEIGHT) {
        fill_rect(frame, x, y, width, 0, color);
    }
    if height > 0 && x == clamp(x, 0, config::SCREEN_WIDTH) {
        fill_rect(frame, x, y, 0, height, color);
    }
    if width > 0 && height > 0 {
        if y + height == clamp(y + height, 0, config::SCREEN_HEIGHT) {
            fill_rect(frame, x, y + height, width, 0, color);
        }
        if x + width == clamp(x + width, 0, config::SCREEN_WIDTH) {
            fill_rect(frame, x + width, y, 0, height, color);
        }
    }
}

#[allow(dead_code)]
/// Draws a single pixel.
pub fn draw_pixel(frame: &mut [u8], x: i32, y: i32, color: &Color) {
    if x == clamp(x, 0, config::SCREEN_WIDTH - 1) && y == clamp(y, 0, config::SCREEN_HEIGHT - 1) {
        let pixel_index = (x + y * config::SCREEN_WIDTH) as usize;
        fill_chunk(frame, pixel_index, pixel_index, color);
    }
}

/// Draws a horizontal or vertical line.
pub fn draw_straight_line(frame: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32, color: &Color) {
    if x1 == x2 || y1 == y2 {
        draw_rect(
            frame,
            min(x1, x2),
            min(y1, y2),
            (x2 - x1).abs(),
            (y2 - y1).abs(),
            color,
        );
    }
}

/// Draws an arbitrary line.
/// Prefer `draw_straight_line()` for horizontal or vertical lines.
pub fn draw_line(frame: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32, color: &Color) {
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };

    let mut x = x1;
    let mut y = y1;
    let mut err = dx + dy;

    loop {
        draw_pixel(frame, x, y, color);
        if x == x2 && y == y2 {
            break;
        }
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

/// Draws a circle.
pub fn fill_circle(frame: &mut [u8], center_x: i32, center_y: i32, radius: i32, color: &Color) {
    let radius_f32 = radius as f32 - 0.5;

    // Do the math for a single quadrant
    for y in center_y - radius..center_y + 1 {
        for x in center_x - radius..center_x + 1 {
            let distance_to_center =
                (((center_x - x).pow(2) + (center_y - y).pow(2)) as f32).sqrt();
            if distance_to_center < radius_f32 {
                // Deduce the entire top and bottom rows
                fill_rect(frame, x, y, (center_x - x) * 2, 0, color);
                fill_rect(
                    frame,
                    x,
                    y + 2 * (center_y - y),
                    (center_x - x) * 2,
                    0,
                    color,
                );
                break;
            }
        }
    }
}

/// Fills the entire frame.
pub fn fill(frame: &mut [u8], color: &Color) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(color);
    }
}

/// Fills a continuous chunk of the frame, using pixel indexes.
/// `pixel_end` is inclusive.
fn fill_chunk(frame: &mut [u8], pixel_start: usize, pixel_end: usize, color: &Color) {
    let frame_start = pixel_start * PIXEL_BYTES;
    let frame_end = (pixel_end + 1) * PIXEL_BYTES;

    if frame_end >= frame_start && frame_end < frame.len() {
        for pixel in frame[frame_start..frame_end + 1].chunks_exact_mut(PIXEL_BYTES) {
            pixel.copy_from_slice(color);
        }
    }
}

pub fn draw_image(frame: &mut [u8], x: i32, y: i32, image: &Image) {
    for row in clamp(y, 0, config::SCREEN_HEIGHT)
        ..clamp(y + image.meta.height, 0, config::SCREEN_HEIGHT - 1)
    {
        let frame_row_index = row * config::SCREEN_WIDTH;
        let x_start = clamp(x, 0, config::SCREEN_WIDTH - 1);
        let x_end = clamp(x + image.meta.width, 0, config::SCREEN_WIDTH - 1);

        let frame_start = (frame_row_index + x_start) as usize * PIXEL_BYTES;
        let frame_end = (frame_row_index + x_end/* + 1*/) as usize * PIXEL_BYTES;

        let image_start = ((row - y) * image.meta.height) as usize * PIXEL_BYTES;
        let image_end = image_start + frame_end - frame_start;
        let image_slice = &image.bytes[image_start..image_end];

        frame[frame_start..frame_end].copy_from_slice(image_slice);
    }
}
