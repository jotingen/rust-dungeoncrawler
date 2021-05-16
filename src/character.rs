use crate::basics::{Abilities, Alignment};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Character {
    pub name: String,
    pub race: String,
    pub age: u32,
    pub alignment: Alignment,
    pub abilities: Abilities,
}

impl Character {
    pub fn new() -> Character {
        Character {
            ..Default::default()
        }
    }
}
