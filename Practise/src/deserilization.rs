use serde::{Deserialize};
use serde_json;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String
}

fn main () {
    let s = String::from(r#"{"name":"raunit","password":"234"}"#);
    // let deserilization_string = serde_json::from_str::<User>(&s);
    let deserilization_string:Result<User,Error> = serde_json::from_str(&s);
    match deserilization_string {
        Ok(user) => println!("Name: {}, Password: {}", user.name, user.password),
        Err(e) => println!("{}",e)
    }
}

//serde_json::from_str is a generic function that can deserialize JSON into any type that implements Deserialize.

