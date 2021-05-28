use crate::character::Character;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Game {
  pub character: Character,
}

impl Game {
  pub fn new() -> Game {
    Game {
      ..Default::default()
    }
  }
    
  pub fn save(&self, file: &str) {
    let game_str = serde_json::to_string_pretty(&self).unwrap();
    fs::write(file, game_str).expect("Unable to save file");
  }
}
