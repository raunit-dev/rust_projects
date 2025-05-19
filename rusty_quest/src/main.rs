// use std::collections::HashMap;
mod player;
mod room;
mod world;

use room::Room;
use world::World;
use player::Player;

fn main() {
    
    let mut game_world = World::new();

    let mut entrance = Room::new(
        "Entrance", 
        "You stand at the entrance of an ancient castle. Cobwebs hang from the ceiling."
    );
    
    let mut hallway = Room::new(
        "Hallway",
        "A long, dimly lit hallway stretches before you. Portraits line the walls."
    );
    
    let mut library = Room::new(
        "Library",
        "Shelves of dusty books surround you. A reading desk sits in the corner."
    );
    
    let mut dungeon = Room::new(
        "Dungeon",
        "A cold, damp dungeon with chains on the walls. Something rattles in the darkness."
    );
    
    let mut kitchen = Room::new(
        "Kitchen",
        "An old kitchen with a hearth. Rusty utensils hang on the walls."
    );
    
    // Add rooms to the world
    game_world.add_room(entrance);
    game_world.add_room(hallway);
    game_world.add_room(library);
    game_world.add_room(dungeon);
    game_world.add_room(kitchen);
    
    // Connect rooms
    game_world.connect_rooms("Entrance", "north", "Hallway", "south");
    game_world.connect_rooms("Hallway", "east", "Library", "west");
    game_world.connect_rooms("Hallway", "west", "Kitchen", "east");
    game_world.connect_rooms("Hallway", "north", "Dungeon", "south");

    let mut player = Player::new( "Entrance");

    println!("Player starts in: {}", player.current_room);

      if let Some(room) = game_world.get_room(&player.current_room) {
        println!("Description: {}", room.description);
        println!("Available exits:");
        for (direction, room_name) in &room.exits {
            println!("  {} leads to {}", direction, room_name);
        }
    }

    if let Some(current_room) = game_world.get_room(&player.current_room) {
        if let Some(next_room) = current_room.exits.get("north") {
            player.move_to(next_room);
            println!("\nPlayer moved to: {}", player.current_room);
        }
    }

}
