pub mod classes;
mod generation;
pub mod races;

use crate::actor::player::classes::Classes;
use crate::actor::player::generation::*;
use crate::actor::player::races::Races;
use crate::actor::{Alignment, Gender, Stats};
use crate::items::weapons::{Weapon, Weapons};
use crate::screen::Screen;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Character {
    pub name: String,
    pub gender: Gender,
    pub race: String,
    pub age: u32,
    pub class: String,
    pub alignment: Alignment,
    pub abilities: Stats,
    pub weapons: Vec<Weapon>,
}

impl Character {
    pub fn new(
        &mut self,
        screen: &mut Screen,
        races: &Races,
        classes: &Classes,
        weapons: &Weapons,
    ) -> Character {
        generate(screen, races, classes, weapons)
    }

    pub fn is_weapon_proficient(
        &self,
        races: &Races,
        classes: &Classes,
        weapons: &Weapons,
        weapon_key: &str,
    ) -> bool {
        //Run through race modifiers to check for weapon proficiencies
        for modifier in races.modifiers(&self.race).iter() {
            if modifier.modifier == "dwarven combat training" {
                //You have proficiency with the battleaxe, handaxe, throwing hammer, and warhammer
                if weapons.value(weapon_key).unwrap().name() == "battleaxe"
                    || weapons.value(weapon_key).unwrap().name() == "handaxe"
                    || weapons.value(weapon_key).unwrap().name() == "throwing hammer"
                    || weapons.value(weapon_key).unwrap().name() == "warhammer"
                {
                    return true;
                }
            }
            if modifier.modifier == "elf weapon training" {
                //You have proficiency with the longsword, shortsword, shortbow, and longbow.
                if weapons.value(weapon_key).unwrap().name() == "longsword"
                    || weapons.value(weapon_key).unwrap().name() == "shortsword"
                    || weapons.value(weapon_key).unwrap().name() == "shortbow"
                    || weapons.value(weapon_key).unwrap().name() == "longbow"
                {
                    return true;
                }
            }
            if modifier.modifier == "drow weapon training" {
                //You have proficiency with rapiers, shortswords, and hand crossbows.
                if weapons.value(weapon_key).unwrap().name() == "rapier"
                    || weapons.value(weapon_key).unwrap().name() == "shortsword"
                    || weapons.value(weapon_key).unwrap().name() == "hand crossbow"
                {
                    return true;
                }
            }
        }
        //Run through class to check for weapon proficiencies
        for weapon_proficiency in classes.weapon_proficiencies(&self.class).iter() {
            if &weapons.value(weapon_key).unwrap().name() == weapon_proficiency
                || &weapons.value(weapon_key).unwrap().proficiency() == weapon_proficiency
            {
                return true;
            }
        }

        false
    }
}
