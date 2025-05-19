use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        self.exits.insert(direction.to_string(), room_name.to_string());
    }
    
    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }
    
    pub fn remove_item(&mut self, item: &str) -> Option<String> {
        if let Some(pos) = self.items.iter().position(|i| i == item) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
    
    pub fn has_item(&self, item: &str) -> bool {
        self.items.iter().any(|i| i == item)
    }
}