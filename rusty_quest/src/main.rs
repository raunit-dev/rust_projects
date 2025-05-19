
use std::collections::HashMap;
mod room;
mod world;
mod player;
mod command;

use room::Room;
use world::World;
use player::Player;
use command::game_loop;

fn main() {
    // Initialize the world
    let mut game_world = World::new();
    
    // Create rooms
    let mut entrance = Room::new(
        "Entrance", 
        "You stand at the entrance of an ancient castle. Cobwebs hang from the ceiling."
    );
    // Add items to entrance
    entrance.add_item("lantern");
    entrance.add_item("old map");
    
    let mut hallway = Room::new(
        "Hallway",
        "A long, dimly lit hallway stretches before you. Portraits line the walls."
    );
    // Add items to hallway
    hallway.add_item("dusty key");
    
    let mut library = Room::new(
        "Library",
        "Shelves of dusty books surround you. A reading desk sits in the corner."
    );
    // Add items to library
    library.add_item("ancient book");
    library.add_item("quill pen");
    
    let mut dungeon = Room::new(
        "Dungeon",
        "A cold, damp dungeon with chains on the walls. Something rattles in the darkness."
    );
    // Add items to dungeon
    dungeon.add_item("broken shackle");
    
    let mut kitchen = Room::new(
        "Kitchen",
        "An old kitchen with a hearth. Rusty utensils hang on the walls."
    );
    // Add items to kitchen
    kitchen.add_item("rusty knife");
    kitchen.add_item("wooden bowl");
    
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
    
    // Create a player
    let mut player = Player::new("Adventurer", "Entrance");
    
    // Start the game loop
    game_loop(&mut player, &mut game_world);
}