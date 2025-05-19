use crate::player::Player;
use crate::world::World;
use std::io;

pub enum Command {
    Go(String),
    Look,
    Inventory,
    Take(String),
    Help,
    Quit,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
    let mut words = input.trim().split_whitespace();

    match words.next() {
        Some("go") => {
             if let Some(direction) = words.next() {
                Command::Go(direction.to_lowercase())
             } else {
                Command::Unknown("Go where?".to_string())
             }
        },
        Some("look") => Command::Look,
        Some("inventory") => Command::Inventory,
        Some("take") => {
            let item = words.collect::<Vec<&str>>().join(" ");
            if item.is_empty() {
                Command::Unknown("Take what?".to_string())
            } else {
                Command::Take(item)
            }
        },
        Some("help") => Command::Help,
        Some("quit") => Command::Quit,
        Some(cmd) => Command::Unknown(format!("I don't understand '{}",cmd)),
        None => Command::Unknown("Please enter a command.".to_string()),
    }
}

pub fn process_command(command: Command, player: &mut Player, world: &World) -> Result<bool, String> {
    match command {
        Command::Go(direction) => {

            if let Some(current_room) = world.get_room(&player.current_room) {
                if let Some(next_room) = current_room.exits.get(&direction) {
                    player.move_to(next_room);
                    display_room(player, world);
                    Ok(true)
                } else {
                    Err(format!("There is no exit to the {}", direction))
                }
            } else {
                Err("You're in a void. This is a bug.".to_string())
            }
        },
        Command::Look => {
            display_room(player, world);
            Ok(true)
        },
        Command::Inventory => {
            println!("You are carrying:");
            if player.inventory.is_empty() {
                println!(" Nothing");
            } else {
                for item in &player.inventory {
                    println!(" {}", item);
                }
            }
            Ok(true)
        },
        Command::Take(item_name) => {
            Err(format!("You can't take {} yet. Items will be implemented later.", item_name))
        },
        Command::Help => {
            print_help();
            Ok(true)
        },
        Command::Quit => {
            println!("Thanks for playing!");
            Ok(false)
        },
        Command::Unknown(message) => {
            Err(message)
        },
    }
}

fn display_room(player: &Player, world: &World) {
    if let Some(room) = world.get_room(&player.current_room) {
        println!("\n== {} ==", room.name);
        println!("{}", room.description);
        
        println!("\nExits:");
        if room.exits.is_empty() {
            println!("  None (you're trapped!)");
        } else {
            for (direction, _) in &room.exits {
                println!("  {}", direction);
            }
        }
    } else {
        println!("Error: You're in a room that doesn't exist!");
    }
}

fn print_help() {
    println!("\n== HELP ==");
    println!("Available commands:");
    println!("  go <direction> - Move in the specified direction");
    println!("  look - Look around the current room");
    println!("  inventory - Check what you're carrying");
    println!("  take <item> - Pick up an item (not implemented yet)");
    println!("  help - Display this help message");
    println!("  quit - Exit the game");
}

pub fn game_loop(player: &mut Player, world: &World) {
    let mut running = true;
    
    println!("\nWelcome to the Text Adventure!");
    println!("Type 'help' for available commands.");
    display_room(player, world);
    
    while running {
        print!("\n> ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }
        
        let command = parse_command(&input);
        match process_command(command, player, world) {
            Ok(continue_running) => {
                running = continue_running;
            },
            Err(message) => {
                println!("{}", message);
            }
        }
    }
}
