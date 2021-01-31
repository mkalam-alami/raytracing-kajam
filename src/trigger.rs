use crate::{map::Map, tileset::Tileset};

pub fn trigger(map: &mut Map, door: &(usize, usize), trigger_color_id: u8) {
  let door_value = Tileset::get_door_value(*map.get(door.0, door.1).unwrap()).unwrap();

  if trigger_color_id == 2 { // decrease by 1
    if door_value >= 1 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value - 1));
    }
  }
  else if trigger_color_id == 3 { // decrease by 2
    if door_value >= 2 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value - 2));
    }
  }
  else if trigger_color_id == 4 { // decrease by 3
    if door_value >= 3 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value - 3));
    }
  }

  else if trigger_color_id == 1 { // increase by 7
    if door_value <= 2 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value + 7));
    }
  }
  else if trigger_color_id == 5 { // increase by 1
    if door_value <= 8 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value + 1));
    }
  }
  else if trigger_color_id == 6 { // increase by 2
    if door_value <= 7 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value + 2));
    }
  }
  else if trigger_color_id == 7 { // increase by 3
    if door_value <= 6 {
      map.set(door.0, door.1, Tileset::get_door_color_id(door_value + 3));
    }
  }
}
