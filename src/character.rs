use crate::basics::{Abilities, Alignment};
use crate::classes::Classes;
use crate::races::Races;
use crate::utils::*;
use crate::weapons::{Weapon, Weapons};
use serde::{Deserialize, Serialize};
use sm::sm;

sm! {
    CharacterCreationState {
        InitialStates { Idle }

        ChooseSex {
            Idle, Summary => Sex
        }

        ChooseRace {
            Sex, Race => Race
        }

        ChooseClass {
            Race, Class => Class
        }

        ChooseStats {
            Class => Stats
        }

        ChooseEquipment {
            Stats => Equipment
        }

        ChooseName {
            Equipment => Name
        }

        ChooseSummary {
            Name => Summary
        }

        Done {
            Summary => Finished
        }
    }
}
use crate::character::CharacterCreationState::{Variant::*, *};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Character {
    pub name: String,
    pub race: String,
    pub age: u32,
    pub class: String,
    pub alignment: Alignment,
    pub abilities: Abilities,
    pub weapon: Weapon,
}

fn roll_stats(rolls: &mut [u8; 6]) {
    println!("Rolling own");
    for roll in rolls.iter_mut() {
        let mut die_rolls: [u8; 4] = [d(6), d(6), d(6), d(6)];
        die_rolls.sort_unstable();
        die_rolls.reverse();
        *roll = die_rolls[0] + die_rolls[1] + die_rolls[2];
        println!("Rolled {:?} for {}", die_rolls, *roll);
    }
    rolls.sort_unstable();
    rolls.reverse();
}

impl Character {
    pub fn new() -> Character {
        Character {
            ..Default::default()
        }
    }

    pub fn character_creation(&mut self, races: &Races, classes: &Classes, weapons: &Weapons) {
        let mut stats: [u8; 6];

        let mut sm = Machine::new(Idle).as_enum();
        loop {
            sm = match sm {
                InitialIdle(m) => {
                    clear();

                    println!("Character Creation\n\n");

                    pause();

                    m.transition(ChooseSex).as_enum()
                }
                SexByChooseSex(m) => {
                    clear();

                    println!("Choose Sex:\n\n");

                    pause();

                    m.transition(ChooseRace).as_enum()
                }
                RaceByChooseRace(m) => {
                    clear();

                    println!("Choose race:");

                    for (count, race_key) in races.keys().iter().enumerate() {
                        println!("{:>2}) {}", count + 1, races.detail_race(&race_key).trim());
                    }

                    let number = pick_number(
                        "Choose race, leave blank for random.",
                        1,
                        races.keys().len() as u32,
                    ) - 1;

                    println!("{}", races.details(&races.keys()[number as usize]));

                    if pick_yes_or_no("Use this race?") {
                        self.race = races.keys()[number as usize].to_string();
                        m.transition(ChooseClass).as_enum()
                    } else {
                        m.transition(ChooseRace).as_enum()
                    }
                }
                ClassByChooseClass(m) => {
                    clear();

                    println!("Choose class:");

                    for (count, class_key) in classes.keys().iter().enumerate() {
                        println!(
                            "{:>2}) {}",
                            count + 1,
                            classes.detail_class(&class_key).trim()
                        );
                    }

                    let number = pick_number(
                        "Choose class, leave blank for random.",
                        1,
                        classes.keys().len() as u32,
                    ) - 1;

                    println!("{}", classes.details(&classes.keys()[number as usize]));

                    pause();

                    if pick_yes_or_no("Use this class?") {
                        self.class = classes.keys()[number as usize].to_string();
                        m.transition(ChooseStats).as_enum()
                    } else {
                        m.transition(ChooseClass).as_enum()
                    }
                }
                StatsByChooseStats(m) => {
                    clear();

                    println!("Roll stats\n");

                    stats = [15, 14, 13, 12, 10, 8];
                    println!("Default stats are {:?}", stats);

                    if pick_yes_or_no("Roll your own stats?") {
                        roll_stats(&mut stats);
                    }

                    println!("Using stats {:?}", stats);

                    pause();

                    m.transition(ChooseEquipment).as_enum()
                }
                EquipmentByChooseEquipment(m) => {
                    clear();

                    println!("Choose equipment:");

                    for (count, weapon_key) in weapons.keys().iter().enumerate() {
                        println!(
                            "{:>2}) {} {}",
                            count + 1,
                            if self.is_weapon_proficient(races, classes, weapons, weapon_key) {
                                "*"
                            } else {
                                " "
                            },
                            weapons.detail_weapon(&weapon_key).trim()
                        );
                    }

                    pause();

                    m.transition(ChooseName).as_enum()
                }
                NameByChooseName(m) => {
                    clear();

                    println!("Choose name:");

                    self.name = "Boaty McBoatface".to_string();

                    pause();

                    m.transition(ChooseSummary).as_enum()
                }
                SummaryByChooseSummary(m) => {
                    clear();

                    println!("Character Summary:");

                    //name

                    //race

                    //age
                    self.age = 25;

                    //class

                    //alignment
                    self.alignment = Alignment::N;

                    //ability_score_base
                    dbg!(races.ability_score_increase(&self.race));
                    self.abilities.strength =
                        races.ability_score_increase(&self.race).abilities.strength;
                    self.abilities.dexterity =
                        races.ability_score_increase(&self.race).abilities.dexterity;
                    self.abilities.charisma =
                        races.ability_score_increase(&self.race).abilities.charisma;
                    self.abilities.constitution = races
                        .ability_score_increase(&self.race)
                        .abilities
                        .constitution;
                    self.abilities.intellect =
                        races.ability_score_increase(&self.race).abilities.intellect;
                    self.abilities.wisdom =
                        races.ability_score_increase(&self.race).abilities.wisdom;

                    dbg!(&self);

                    pause();

                    if pick_yes_or_no("Use this character?") {
                        m.transition(Done).as_enum()
                    } else {
                        m.transition(ChooseSex).as_enum()
                    }
                }
                FinishedByDone(_) => {
                    break;
                }
            }
        }
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
                if weapons.weapon(weapon_key) == "battleaxe"
                    || weapons.weapon(weapon_key) == "handaxe"
                    || weapons.weapon(weapon_key) == "throwing hammer"
                    || weapons.weapon(weapon_key) == "warhammer"
                {
                    return true;
                }
            }
            if modifier.modifier == "elf weapon training" {
                //You have proficiency with the longsword, shortsword, shortbow, and longbow.
                if weapons.weapon(weapon_key) == "longsword"
                    || weapons.weapon(weapon_key) == "shortsword"
                    || weapons.weapon(weapon_key) == "shortbow"
                    || weapons.weapon(weapon_key) == "longbow"
                {
                    return true;
                }
            }
            if modifier.modifier == "drow weapon training" {
                //You have proficiency with rapiers, shortswords, and hand crossbows.
                if weapons.weapon(weapon_key) == "rapier"
                    || weapons.weapon(weapon_key) == "shortsword"
                    || weapons.weapon(weapon_key) == "hand crossbow"
                {
                    return true;
                }
            }
        }
        //Run through class to check for weapon proficiencies
        for weapon_proficiency in classes.weapon_proficiencies(&self.class).iter() {
            if weapons.weapon(weapon_key) == weapon_proficiency.to_string()
                || weapons.proficiency(weapon_key) == weapon_proficiency.to_string()
            {
                return true;
            }
        }

        return false;
    }
}
