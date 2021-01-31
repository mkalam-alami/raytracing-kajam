use crate::{core::{config::TARGET_FPS, game::GameState, math::clamp}, map::Map, point::Point, tileset::Tileset};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const MOV_SPEED: f32 = 0.09;
const ROT_SPEED: f32 = 4.0;
const PLAYER_SIZE: f32 = 0.4 / MOV_SPEED;
const TRIGGER_PERIOD: f32 = 0.5;
const PASSIVE_SNAP: f32 = 0.20;
const ACTIVE_SNAP: f32 = 0.13;
const MAX_THIRD_PERSON_OFFSET: f32 = 0.8;

#[derive(Clone)]
pub struct Player {
    pub pos: Point,
    pub dir: Point,
    pub cell_last_triggered: u32,
    pub is_triggering_this_update: bool,
    pub third_person_offset: f32
}

impl Player {
    pub fn new(pos: Point, dir: Point) -> Self {
        Self {
            pos: pos + Point::new(0.5, 0.5)/* tile center*/ + (dir * -0.49) /* intro mini-anim */,
            dir,
            cell_last_triggered: 0,
            is_triggering_this_update: false,
            third_person_offset: MAX_THIRD_PERSON_OFFSET
        }
    }

    pub fn get_camera_plane(&self) -> Point {
        self.dir.rotate(90.) * 0.66
    }

    pub fn get_current_cell(&self) -> (usize, usize) {
        (self.pos.x.floor() as usize, self.pos.y.floor() as usize)
    }

    pub fn update(&mut self, map: &Map, input: &WinitInputHelper, game_state: &GameState) {
        let current_cell = self.get_current_cell();
        self.third_person_offset = clamp(self.third_person_offset + 0.2, 0.0, MAX_THIRD_PERSON_OFFSET);

        if input.key_held(VirtualKeyCode::Left) {
            self.dir = self.dir.rotate(-ROT_SPEED);
            self.third_person_offset /= 1.2;
        }
        if input.key_held(VirtualKeyCode::Right) {
            self.dir = self.dir.rotate(ROT_SPEED);
            self.third_person_offset /= 1.2;
        }

        let mut dpos = Point::new(0., 0.);
        if input.key_held(VirtualKeyCode::Up) {
            dpos = self.dir * MOV_SPEED;
        }
        if input.key_held(VirtualKeyCode::Down) {
            dpos = self.dir * -MOV_SPEED;
        }

        let dx_collision = map.get(
            (self.pos.x + (dpos.x * PLAYER_SIZE)) as usize,
            self.pos.y as usize,
        );
        if dx_collision.is_some() && !Tileset::is_colliding(*dx_collision.unwrap()) {
            self.pos.x = self.pos.x + dpos.x;
        }

        let dy_collision = map.get(
            self.pos.x as usize,
            (self.pos.y + (dpos.y * PLAYER_SIZE)) as usize,
        );
        if dy_collision.is_some() && !Tileset::is_colliding(*dy_collision.unwrap()) {
          self.pos.y = self.pos.y + dpos.y;
        }

        if self.get_current_cell() != current_cell
            || (game_state.frame_counter - self.cell_last_triggered) as f32 >= TARGET_FPS as f32 * TRIGGER_PERIOD {
          self.cell_last_triggered = game_state.frame_counter;
          self.is_triggering_this_update = true;
        } else {
            self.is_triggering_this_update = false;
        }

        // snap to cell center
        let snap_pos = self.get_current_cell();
        let snap_strength: f32 = if dpos.get_magnitude() > 0.0 { ACTIVE_SNAP } else { PASSIVE_SNAP };
        self.pos = Point {
          x: (self.pos.x + (snap_pos.0 as f32 + 0.5) * snap_strength) / (1. + snap_strength),
          y: (self.pos.y + (snap_pos.1 as f32 + 0.5) * snap_strength) / (1. + snap_strength),
        }
    }
}
