use crate::COLUMN_WIDTH;
use serde::{Deserialize, Serialize};
use textwrap;

#[derive(Debug, Default, Serialize, Deserialize)]
struct WeaponProperties {
    proficiency: String,
    class: String,
    finesse: bool,
    heavy: bool,
    light: bool,
    loading: bool,
    ammunition: bool,
    range_normal: u32,
    range_max: u32,
    reach: bool,
    special: bool,
    thrown: bool,
    two_handed: bool,
    versatile: bool,
    versatile_dmg: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Weapon {
    weapon: String,
    cost: u32,
    damage: String,
    weight: f32,
    properties: WeaponProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapons {
    weapons: Vec<Weapon>,
}

impl Weapons {
    pub fn new() -> Weapons {
        let weapon_json = include_str!("weapons.json");
        let weapons: Vec<Weapon> = serde_json::from_str(&weapon_json).unwrap();
        Weapons { weapons }
    }
}
