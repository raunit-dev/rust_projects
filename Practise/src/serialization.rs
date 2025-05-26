use serde::{Serialize,Deserialize};

#[derive(Copy,Clone)]
struct User {
    username: String,
    password: String
}

fn main() {
    let u = User {
        username: String::from("JAISWAL"),
        password: String::from("Raunit")
    };

    let serialize_string = serde_json::to_string(&u);
    match serialize_string {
        Ok(json) => println!("Serialized: {}", json),
        Err(e) => println!("Serialization error: {}", e),
    }
}