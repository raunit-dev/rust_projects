use crate::player::Player;
use crate::world::World;
use std::io;

pub enum Command {
    Go(String),
    Look,
    Inventory,
    Take(String),
    Drop(String),
    Use(String),
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
        Some("drop") => {
            let item = words.collect::<Vec<&str>>().join(" ");
            if item.is_empty() {
                Command::Unknown("Drop what?".to_string())
            } else {
                Command::Take(item)
            }
        },
        Some("use") => {
            let item = words.collect::<Vec<&str>>().join(" ");
            if item.is_empty() {
                Command::Unknown("Use what?".to_string())
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
            if let Some(room) = world.get_room_mut(&player.current_room) {
                // Check if the item is in the room
                if room.has_item(&item_name) {
                    // Remove the item from the room and add it to player's inventory
                    if let Some(item) = room.remove_item(&item_name) {
                        player.add_to_inventory(&item);
                        println!("You pick up the {}.", item_name);
                        Ok(true)
                    } else {
                        // This should never happen due to the has_item check
                        Err("Error taking item".to_string())
                    }
                } else {
                    Err(format!("There is no {} here.", item_name))
                }
            } else {
                Err("You're in a void. This is a bug.".to_string())
            }
        },Command::Drop(item_name) => {
            // Check if the player has the item
            if let Some(item_pos) = player.inventory.iter().position(|i| i == &item_name) {
                // Remove it from inventory
                let item = player.inventory.remove(item_pos);
                
                // Add it to the current room
                if let Some(room) = world.get_room_mut(&player.current_room) {
                    room.add_item(&item);
                    println!("You drop the {}.", item);
                    Ok(true)
                } else {
                    // If the room doesn't exist (shouldn't happen), put the item back
                    player.add_to_inventory(&item);
                    Err("You're in a void. This is a bug.".to_string())
                }
            } else {
                Err(format!("You don't have a {}.", item_name))
            }
        },
        Command::Use(item_name) => {
            // Check if the player has the item
            if player.has_item(&item_name) {
                // Implement special effects based on the item
                match item_name.as_str() {
                    "key" => {
                        if player.current_room == "Hallway" {
                            println!("You use the key to unlock a secret door!");
                            // This could trigger some game state change
                            // For example, add a new exit to the room
                            if let Some(room) = world.get_room_mut("Hallway") {
                                room.add_exit("down", "Secret Room");
                                
                                // Create the secret room if it doesn't exist
                                if !world.rooms.contains_key("Secret Room") {
                                    let mut secret_room = crate::room::Room::new(
                                        "Secret Room",
                                        "A hidden chamber with ancient treasures!"
                                    );
                                    secret_room.add_exit("up", "Hallway");
                                    secret_room.add_item("golden crown");
                                    world.add_room(secret_room);
                                }
                                
                                Ok(true)
                            } else {
                                Err("Room error".to_string())
                            }
                        } else {
                            println!("There's nothing to unlock here with the key.");
                            Ok(true)
                        }
                    },
                    "lantern" => {
                        println!("You light the lantern, providing better illumination.");
                        // This could improve visibility or reveal hidden items
                        if player.current_room == "Dungeon" {
                            println!("In the improved light, you notice a hidden key on the floor!");
                            if let Some(room) = world.get_room_mut("Dungeon") {
                                // Only add the key if it's not already there
                                if !room.has_item("rusty key") {
                                    room.add_item("rusty key");
                                }
                            }
                        }
                        Ok(true)
                    },
                    "book" => {
                        println!("You read the book and learn about the castle's history.");
                        // This could provide hints
                        if player.current_room == "Library" {
                            println!("The book mentions a secret door in the hallway that can be opened with a key.");
                        }
                        Ok(true)
                    },
                    _ => {
                        println!("You can't figure out how to use the {} effectively.", item_name);
                        Ok(true)
                    }
                }
            } else {
                Err(format!("You don't have a {}.", item_name))
            }
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
    println!("  inventory (or i) - Check what you're carrying");
    println!("  take <item> - Pick up an item");
    println!("  drop <item> - Drop an item from your inventory");
    println!("  use <item> - Use an item (special effects)");
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
