use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub deadline: String,
    pub done: bool
}