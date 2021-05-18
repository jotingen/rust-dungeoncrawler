use crate::COLUMN_WIDTH;
use serde::{Deserialize, Serialize};
use textwrap;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Class {
    class: String,
    description: String,
    hit_die: u8,
    primary_ability: Vec<String>,
    saving_throw_proficiencies: Vec<String>,
    armor_proficiencies: Vec<String>,
    weapon_proficiencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classes {
    classes: Vec<Class>,
}

impl Classes {
    pub fn new() -> Classes {
        let class_json = include_str!("classes.json");
        let classes: Vec<Class> = serde_json::from_str(&class_json).unwrap();
        Classes { classes }
    }
}
