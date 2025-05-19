use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub inventory: Vec<String>,
    pub current_room: String,
}

impl Player {
    pub fn new(name: &str, starting_room: &str) -> Self {
        Player {
            name: name.to_string(),
            inventory: Vec::new(),
            current_room: starting_room.to_string(),
        }
    }
    
    pub fn move_to(&mut self, room_name: &str) {
        self.current_room = room_name.to_string();
    }
    
    pub fn add_to_inventory(&mut self, item: &str) {
        self.inventory.push(item.to_string());
    }
    
    pub fn has_item(&self, item: &str) -> bool {
        self.inventory.iter().any(|i| i == item)
    }
}