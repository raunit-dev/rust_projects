mod task;
mod manager;
mod commands;

use manager::TaskManager;
use std::io::{self, Write};
use commands::Command;

fn main() {
    let mut manager = TaskManager::new();
    println!("Welcome to Mini Task Manager!");
    println!("Type 'help' for commands.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        match Command::parse(input) {
            Ok(command) => {
                if let Err(e) = command.execute(&mut manager) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(err) => {
                eprintln!("Invalid command: {}", err);
            }
        }
    }
}
