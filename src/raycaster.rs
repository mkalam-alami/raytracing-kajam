use crate::{core::{assets::Image, colors::Color, draw::draw_pixel, math::clamp}, palette::Palette};
use crate::core::config;
use crate::map::Map;
use crate::point::Point;
use crate::player::Player;

#[derive(Clone)]
pub struct Raycaster {
  map: Map,
  palette: Palette
}

#[allow(dead_code)]
impl Raycaster {
  pub fn new(map: Map, palette: Palette) -> Self {
    Self { map, palette }
  }

  pub fn draw(&self, frame: &mut [u8], player: &Player) {
    let screen_size = Point { x: config::SCREEN_WIDTH as f32, y: config::SCREEN_HEIGHT as f32 };
    for x in 0..config::SCREEN_WIDTH {
      let camera_x = 2. * x as f32 / screen_size.x - 1.; // [-1;1]
      let ray_dir = player.dir + player.get_camera_plane() * camera_x;

      let mut map_coords = player.pos.floor();
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
        side_dist.x = (player.pos.x - map_coords.x) * delta_dist.x;
      } else {
        step.x = 1.;
        side_dist.x = (map_coords.x + 1. - player.pos.x) * delta_dist.x;
      }
      if ray_dir.y < 0. {
        step.y = -1.;
        side_dist.y = (player.pos.y - map_coords.y) * delta_dist.y;
      } else {
        step.y = 1.;
        side_dist.y = (map_coords.y + 1. - player.pos.y) * delta_dist.y;
      }

      let mut hit = 0 as i8; // hit wall value
      let mut side = false; // was a NS or a EW wall hit?

      // perform DDA
      while hit == 0 {
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
          if *tile_value > 0 {
            hit = *tile_value as i8;
          }
        } else {
          hit = -1;
        }
      }

      if hit > 0 {
        let perp_wall_dist = if side {
          (map_coords.y - player.pos.y + (1. - step.y) / 2.) / ray_dir.y
        } else {
          (map_coords.x - player.pos.x + (1. - step.x) / 2.) / ray_dir.x
        };
        self.draw_column(frame, x, player.pos, side, perp_wall_dist, ray_dir, screen_size);
      }
    }
  }

  fn draw_column(&self, frame: &mut [u8], x: i32, pos: Point, side: bool, perp_wall_dist: f32, ray_dir: Point, screen_size: Point) {
    let texture = self.palette.textures.get(0).unwrap();

    let tex_x = self.calc_tex_x(texture, pos, side, perp_wall_dist, ray_dir);

    let line_height = screen_size.y / perp_wall_dist;
    let draw_high_y = ((screen_size.y + line_height) / 2.).min(screen_size.y - 1.) as i32;
    let draw_low_y = ((screen_size.y - line_height) / 2.).max(0.) as i32;

    let mut pixel_color: Color = Default::default();
    let step = (texture.meta.height as f32) / line_height;
    let mut tex_pos = (draw_low_y as f32 - screen_size.y / 2. + line_height / 2.) * step;
    for y in draw_low_y..draw_high_y {
      let tex_y = clamp(tex_pos as i32, 0, texture.meta.height - 1);
      tex_pos += step;
      texture.get(tex_x, tex_y, &mut pixel_color);
      draw_pixel(frame, x, y, &pixel_color);
    }
  }

  fn calc_tex_x(&self, texture: &Image, pos: Point, side: bool, perp_wall_dist: f32, ray_dir: Point) -> i32 {
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
