use crate::{core::{assets::Image, colors::Color, draw::{draw_pixel, draw_straight_line}, game::GameState, math::clamp}, tileset::{Tileset}};
use crate::core::config;
use crate::map::Map;
use crate::point::Point;
use crate::player::Player;

const THIRD_PERSON_OFFSET: f32 = 0.8;

#[derive(Clone)]
pub struct Raycaster {
  map: Map,
  tileset: Tileset
}

#[allow(dead_code)]
impl Raycaster {
  pub fn new(map: Map, tileset: Tileset) -> Self {
    Self { map, tileset }
  }

  pub fn draw(&self, frame: &mut [u8], player: &Player, game_state: &GameState) {
    let screen_size = Point { x: config::SCREEN_WIDTH as f32, y: config::SCREEN_HEIGHT as f32 };

    self.draw_floor(frame, player, screen_size, game_state);
    self.draw_walls(frame, player, screen_size);
  }

  pub fn draw_floor(&self, frame: &mut [u8], player: &Player, screen_size: Point, game_state: &GameState) {
    let camera_plane = player.get_camera_plane();
    let mut pixel_color: Color = Default::default();

    for y in config::SCREEN_HEIGHT/2+1..config::SCREEN_HEIGHT {
      // rayDir for leftmost ray (x = 0) and rightmost ray (x = w)
      let ray_dir_0 = Point {
        x: player.dir.x - camera_plane.x,
        y: player.dir.y - camera_plane.y
      };
      let ray_dir_1 = Point {
        x: player.dir.x + camera_plane.x,
        y: player.dir.y + camera_plane.y
      };

      let p = (y - config::SCREEN_HEIGHT / 2) as f32; // Current y position compared to the center of the screen (the horizon)
      let pos_z = 0.5 * screen_size.y; // Vertical position of the camera.
      let row_distance = pos_z / p;  // Horizontal distance from the camera to the floor for the current row. 0.5 is the z position exactly in the middle between floor and ceiling.

      // calculate the real world step vector we have to add for each x (parallel to camera plane)
      // adding step by step avoids multiplications with a weight in the inner loop
      let floor_step = Point {
        x: row_distance * (ray_dir_1.x - ray_dir_0.x) / screen_size.x,
        y: row_distance * (ray_dir_1.y - ray_dir_0.y) / screen_size.x
      };

      // real world coordinates of the leftmost column. This will be updated as we step to the right.
      let mut floor = Point {
        x: player.pos.x + row_distance * ray_dir_0.x,
        y: player.pos.y + row_distance * ray_dir_0.y
      };
      floor -= player.dir * THIRD_PERSON_OFFSET; // 3rd person view

      for x in 0..config::SCREEN_WIDTH {
        let cell_x = floor.x.floor() as i32;
        let cell_y = floor.y.floor() as i32;

        floor += floor_step;

        if (cell_x, cell_y) == player.get_current_cell() && game_state.frame_counter - player.in_cell_since < (0.25 * 60.0/*FPS*/) as u32  {
          // white flash on cell change
          pixel_color.copy_from_slice(&config::COLOR_WHITE);
        } else {
          let color_id = *self.map.get(cell_x as usize, cell_y as usize).unwrap_or(&Tileset::default_floor_color_id()) as i8;
          if Tileset::is_textured(color_id) {
            if Tileset::is_rendered_as_floor(color_id) {
              let texture = self.tileset.get_texture(color_id);
              // get the texture coordinate from the fractional part
              let tex_x = clamp((texture.meta.width as f32 * (floor.x - cell_x as f32)) as i32, 0, texture.meta.width - 1);
              let tex_y = clamp((texture.meta.height as f32 * (floor.y - cell_y as f32)) as i32, 0, texture.meta.height - 1);
              texture.get(tex_x, tex_y, &mut pixel_color);
            } else {
              self.tileset.pick(Tileset::default_floor_color_id() as usize, &mut pixel_color);
            }
          } else {
            self.tileset.pick(color_id as usize, &mut pixel_color);
          }
        }
        draw_pixel(frame, x, y, &pixel_color);
      }
    }

  }

  pub fn draw_walls(&self, frame: &mut [u8], player: &Player, screen_size: Point) {
    let camera_pos = player.pos - player.dir * THIRD_PERSON_OFFSET; // 3rd person view

    for x in 0..config::SCREEN_WIDTH {
      let camera_x = 2. * x as f32 / screen_size.x - 1.; // [-1;1]
      let ray_dir = player.dir + player.get_camera_plane() * camera_x;

      let mut map_coords = camera_pos.floor();
      let delta_dist = Point::new(
        if ray_dir.y == 0. {
          0.
        } else if ray_dir.x == 0. {
          1.
        } else {
          1. / ray_dir.x.abs()
        },
        if ray_dir.x == 0. {
          0.
        } else if ray_dir.y == 0. {
          1.
        } else {
          1. / ray_dir.y.abs()
        },
      ); // length of ray from one x or y-side to next x or y-side

      let mut step = Point::new(0., 0.); // what direction to step in x or y-direction (either +1 or -1)
      let mut side_dist = Point::new(0., 0.); //length of ray from current position to next x or y-side
      if ray_dir.x < 0. {
        step.x = -1.;
        side_dist.x = (camera_pos.x - map_coords.x) * delta_dist.x;
      } else {
        step.x = 1.;
        side_dist.x = (map_coords.x + 1. - camera_pos.x) * delta_dist.x;
      }
      if ray_dir.y < 0. {
        step.y = -1.;
        side_dist.y = (camera_pos.y - map_coords.y) * delta_dist.y;
      } else {
        step.y = 1.;
        side_dist.y = (map_coords.y + 1. - camera_pos.y) * delta_dist.y;
      }

      let mut hit = -1 as i8; // hit wall value
      let mut side = false; // was a NS or a EW wall hit?

      // perform DDA
      while hit == -1 {
        // jump to next map square, OR in x-direction, OR in y-direction
        if side_dist.x < side_dist.y {
          side_dist.x += delta_dist.x;
          map_coords.x += step.x;
          side = false;
        } else {
          side_dist.y += delta_dist.y;
          map_coords.y += step.y;
          side = true;
        }

        // check if ray has hit a wall
        let tile = self.map.get(map_coords.x as usize, map_coords.y as usize);
        if let Some(tile_value) = tile {
          if Tileset::is_rendered_as_wall(*tile_value as i8) {
            hit = *tile_value as i8;
          }
        } else {
          hit = Tileset::default_wall_color_id() as i8;
        }
      }

      if Tileset::is_rendered_as_wall(hit) {
        let perp_wall_dist = if side {
          (map_coords.y - camera_pos.y + (1. - step.y) / 2.) / ray_dir.y
        } else {
          (map_coords.x - camera_pos.x + (1. - step.x) / 2.) / ray_dir.x
        };
        self.draw_wall_column(frame, hit, x, camera_pos, side, perp_wall_dist, ray_dir, screen_size);
      }
    }
  }

  fn draw_wall_column(&self, frame: &mut [u8], hit: i8, x: i32, pos: Point, side: bool, perp_wall_dist: f32, ray_dir: Point, screen_size: Point) {
    let line_height = screen_size.y / perp_wall_dist;
    let draw_high_y = ((screen_size.y + line_height) / 2.).min(screen_size.y - 1.) as i32;
    let draw_low_y = ((screen_size.y - line_height) / 2.).max(0.) as i32;

    if Tileset::is_textured(hit) {
      let texture = self.tileset.get_texture(hit);

      let tex_x = self.calc_wall_tex_x(texture, pos, side, perp_wall_dist, ray_dir);

      let mut pixel_color: Color = Default::default();
      let step = (texture.meta.height as f32) / line_height;
      let mut tex_pos = (draw_low_y as f32 - screen_size.y / 2. + line_height / 2.) * step;
      for y in draw_low_y..draw_high_y {
        let tex_y = clamp(tex_pos as i32, 0, texture.meta.height - 1);
        tex_pos += step;
        texture.get(tex_x, tex_y, &mut pixel_color);
        draw_pixel(frame, x, y, &pixel_color);
      }
    } else {
      let mut color: Color = Default::default();
      self.tileset.pick(hit as usize, &mut color);
      draw_straight_line(frame, x, draw_low_y, x, draw_high_y, &color);
    }
  }

  fn calc_wall_tex_x(&self, texture: &Image, pos: Point, side: bool, perp_wall_dist: f32, ray_dir: Point) -> i32 {
    let mut wall_x: f32;
    if side {
      wall_x = pos.x + perp_wall_dist * ray_dir.x;
    } else {
      wall_x = pos.y + perp_wall_dist * ray_dir.y;
    }
    wall_x = wall_x.fract();

    // x coordinate on the texture
    let mut tex_x = (wall_x * texture.meta.width as f32) as i32;
    if (side && ray_dir.y < 0.) || (!side && ray_dir.x > 0.) {
      tex_x = texture.meta.width - tex_x - 1;
    }

    clamp(tex_x, 0, texture.meta.width - 1)
  }
}
