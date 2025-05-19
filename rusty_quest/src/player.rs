pub struct Player {
    pub inventory: Vec<String>,
    pub current_room: String,
}

impl Player {
    pub fn new(starting_room: &str) -> Self {
        Player {
            inventory: Vec::new(),
            current_room: starting_room.to_string(),
        }
    }

    pub fn move_to(&mut self, room_name: &str) {
        self.current_room = room_name.to_string();
    }
}