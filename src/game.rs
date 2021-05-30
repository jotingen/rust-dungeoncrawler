use crate::character::Character;
use crate::levels::Levels;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Game {
    pub character: Character,
    pub levels: Levels,
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

    pub fn load(&mut self, file: &str) {
        let game_str = fs::read_to_string(&file).expect("Unable to open file");
        *self = serde_json::from_str(&game_str).unwrap();
    }
}
