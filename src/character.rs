use crate::basics::{Abilities, Alignment, Gender};
use crate::classes::Classes;
use crate::races::Races;
use crate::utils::*;
use crate::weapons::{Weapon, Weapons};
use crate::COLUMN_WIDTH;
use serde::{Deserialize, Serialize};
use sm::sm;
use textwrap;

sm! {
    CharacterCreationState {
        InitialStates { Idle }

        ChooseGender {
            Idle, Summary, Gender => Gender
        }

        ChooseRace {
            Gender, Race => Race
        }

        ChooseClass {
            Race, Class => Class
        }

        ChooseStats {
            Class => Stats
        }

        ChooseEquipment {
            Stats, Equipment => Equipment
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
    pub gender: Gender,
    pub race: String,
    pub age: u32,
    pub class: String,
    pub alignment: Alignment,
    pub abilities: Abilities,
    pub weapons: Vec<Weapon>,
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

                    m.transition(ChooseGender).as_enum()
                }
                GenderByChooseGender(m) => {
                    clear();

                    println!("Choose Gender:\n\n");

                    println!(" 1) Male");
                    println!(" 2) Female");

                    let number = pick_number("Choose gender, leave blank for random.", 1, 2) - 1;

                    println!("{}", if number == 0 { "M" } else { "F" });

                    if pick_yes_or_no("Use this gender?") {
                        self.gender = if number == 0 { Gender::M } else { Gender::F };
                        m.transition(ChooseRace).as_enum()
                    } else {
                        m.transition(ChooseGender).as_enum()
                    }
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

                    let mut weapon_list: String = "".to_string();
                    for (count, weapon_key) in weapons.keys().iter().enumerate() {
                        weapon_list = format!(
                            "{}{:>2}) {} {}\n",
                            weapon_list,
                            count + 1,
                            if self.is_weapon_proficient(races, classes, weapons, weapon_key) {
                                "*"
                            } else {
                                " "
                            },
                            weapons.value(&weapon_key).unwrap().detail_name().trim()
                        );
                    }
                    for line in
                        textwrap::wrap_columns(&weapon_list, 3, COLUMN_WIDTH, "", "", "").iter()
                    {
                        println!("{}", line);
                    }

                    let number = pick_number(
                        "Choose weapon, leave blank for random.",
                        1,
                        weapons.keys().len() as u32,
                    ) - 1;

                    println!(
                        "{}",
                        weapons
                            .value(&weapons.keys()[number as usize])
                            .unwrap()
                            .details()
                    );

                    if pick_yes_or_no("Use this weapon?") {
                        self.weapons.push(
                            weapons
                                .weapon(&weapons.keys()[number as usize].to_string())
                                .unwrap(),
                        );
                        m.transition(ChooseName).as_enum()
                    } else {
                        m.transition(ChooseEquipment).as_enum()
                    }
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
                        m.transition(ChooseGender).as_enum()
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
            if weapons.value(weapon_key).unwrap().name() == weapon_proficiency.to_string()
                || weapons.value(weapon_key).unwrap().proficiency()
                    == weapon_proficiency.to_string()
            {
                return true;
            }
        }

        return false;
    }
}
