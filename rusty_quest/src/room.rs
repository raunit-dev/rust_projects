use std::collections::HashMap;
pub struct Room {
    pub name: String,
    pub description: String,
    pub items: Vec<String>,
    pub exits: HashMap<String, String>, // direction -> room_name
}

impl Room {
    pub fn new(name: &str, description: &str) -> Self {
        Room {
            name: name.to_string(),
            description: description.to_string(),
            exits: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn add_exit(&mut self, direction: &str, room_name: &str) {
        self.exits.insert(direction.to_string(), room_name.to_string() );
    }
}