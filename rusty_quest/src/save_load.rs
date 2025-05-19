use std::fs::File;
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};

use crate::player::Player;
use crate::world::World;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub world: World,
}

pub fn save_game(player: &Player, world: &World, filename: &str) -> io::Result<()> {
    let game_state = GameState {
        player: player.clone(),
        world: world.clone(),
    };
    
    let json = serde_json::to_string_pretty(&game_state)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}

pub fn load_game(filename: &str) -> io::Result<GameState> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let game_state: GameState = serde_json::from_str(&contents)?;
    
    Ok(game_state)
}