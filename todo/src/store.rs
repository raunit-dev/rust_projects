use std::fs::File;
use std::io::{self,Write,Read};
use serde_json;
use crate::models::Todo;

pub fn save_to_file(todos: &Vec<Todo>) -> io::Result<()> {
    let json_data = serde_json::to_string(todos).unwrap();

    let mut file = File::create("todo.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

pub fn load_from_file() -> io::Result<Vec<Todo>> {
    let mut file = File::open("todo.json")?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    let todos: Vec<Todo> = serde_json::from_str(&json_data)?;

    Ok(todos)
}