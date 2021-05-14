use crate::basics::{Abilities, Alignment};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Character {
    name: String,
    race: String,
    age: u32,
    alignment: Alignment,
    ability_score_base: Abilities,
}

impl Character {
    pub fn new() -> Character {
        Character {
            ..Default::default()
        }
    }
}
