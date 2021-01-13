use crate::core::draw::draw_straight_line;
use crate::core::config;
use crate::map::Map;
use crate::point::Point;
use crate::core::colors;

#[derive(Clone)]
pub struct Raycaster {
  map: Map,
  position: Point,
  direction: Point,
  camera_plane: Point,
}

#[allow(dead_code)]
impl Raycaster {
  pub fn new(map: Map) -> Self {
    Self {
      map,
      position: Point { x: 22., y: 22. },
      direction: Point { x: -1., y: 0. },
      camera_plane: Point { x: 0., y: 0.66 },
    }
  }

  pub fn draw(&self, frame: &mut [u8]) {
    let screen_size = Point { x: config::SCREEN_WIDTH as f32, y: config::SCREEN_HEIGHT as f32 };
    for x in 0..config::SCREEN_WIDTH {
      let camera_x = 2. * x as f32 / screen_size.x - 1.; // [-1;1]
      let ray_dir = self.direction + self.camera_plane * camera_x;

      let mut map_coords = self.position.floor();
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
        side_dist.x = (self.position.x - map_coords.x) * delta_dist.x;
      } else {
        step.x = 1.;
        side_dist.x = (map_coords.x + 1. - self.position.x) * delta_dist.x;
      }
      if ray_dir.y < 0. {
        step.y = -1.;
        side_dist.y = (self.position.y - map_coords.y) * delta_dist.y;
      } else {
        step.y = 1.;
        side_dist.y = (map_coords.y + 1. - self.position.y) * delta_dist.y;
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
          (map_coords.y - self.position.y + (1. - step.y) / 2.) / ray_dir.y
        } else {
          (map_coords.x - self.position.x + (1. - step.x) / 2.) / ray_dir.x
        };
        let line_height = screen_size.y / perp_wall_dist;
        let draw_start_y = ((screen_size.y + line_height) / 2.).min(screen_size.y - 1.);
        let draw_end_y = ((screen_size.y - line_height) / 2.).max(0.);

        let color = match hit {
          4 => colors::COLOR_WHITE,
          1 => colors::COLOR_YELLOW,
          _ => colors::COLOR_PURPLE
        };

        draw_straight_line(frame, x as i32, draw_start_y as i32, x as i32, draw_end_y as i32, &color);
      }
    }
  }
}
