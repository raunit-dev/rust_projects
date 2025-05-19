use std::collections::HashMap;
use crate::room::Room;

pub struct World {
    pub rooms: HashMap<String, Room>,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name.clone(), room);
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    pub fn get_room_mut(&mut self,room_name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_name)
    }

    pub fn connect_rooms(&mut self, room1: &str, dir1: &str, room2: &str, dir2: &str) {
        if let Some(room) = self.get_room_mut(room1) {
            room.add_exit(dir1,room2)
        }
        if let Some(room) = self.get_room_mut(room2) {
            room.add_exit(dir2, room1);
        }
    }

}