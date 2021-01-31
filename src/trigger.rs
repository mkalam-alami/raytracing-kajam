use crate::{map::Map, tileset::Tileset};

pub fn trigger(map: &mut Map, door: &(usize, usize), trigger_color_id: u8) {
  let door_value = Tileset::get_door_value(*map.get(door.0, door.1).unwrap()).unwrap();

  if trigger_color_id == 2 { // Purple = decrease by 1
    if door_value > 0 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value - 1));
    }
  }
}
