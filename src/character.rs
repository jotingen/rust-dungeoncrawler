use crate::basics::{Abilities, Alignment};
use crate::classes::Classes;
use crate::races::Races;
use crate::utils::*;
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

    pub fn character_creation(races: &Races, classes: &Classes) -> Character {
        let mut character: Character = Character::new();
        let mut stats: [u8; 6];
        let mut race: String = "".to_string();
        let mut class: String = "".to_string();
        let mut name: String = "".to_string();

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
                        println!("{:>2}) {}", count + 1, races.race(&race_key));
                    }

                    let number = pick_number(
                        "Choose race, leave blank for random.",
                        1,
                        races.keys().len() as u32,
                    ) - 1;

                    println!("{}", races.details(&races.keys()[number as usize]));

                    if pick_yes_or_no("Use this race?") {
                        race = races.keys()[number as usize].to_string();
                        m.transition(ChooseClass).as_enum()
                    } else {
                        m.transition(ChooseRace).as_enum()
                    }
                }
                ClassByChooseClass(m) => {
                    clear();

                    println!("Choose class:");

                    for (count, class_key) in classes.keys().iter().enumerate() {
                        println!("{:>2}) {}", count + 1, classes.class(&class_key));
                    }

                    let number = pick_number(
                        "Choose class, leave blank for random.",
                        1,
                        classes.keys().len() as u32,
                    ) - 1;

                    println!("{}", classes.details(&classes.keys()[number as usize]));

                    pause();

                    if pick_yes_or_no("Use this class?") {
                        class = classes.keys()[number as usize].to_string();
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

                    pause();

                    m.transition(ChooseName).as_enum()
                }
                NameByChooseName(m) => {
                    clear();

                    println!("Choose name:");

                    name = "Boaty McBoatface".to_string();

                    pause();

                    m.transition(ChooseSummary).as_enum()
                }
                SummaryByChooseSummary(m) => {
                    clear();

                    println!("Character Summary:");

                    character = Character::new();

                    //name
                    character.name = name.to_string();

                    //race
                    character.race = race.to_string();

                    //age
                    character.age = 25;

                    //class
                    character.class = class.to_string();

                    //alignment
                    character.alignment = Alignment::N;

                    //ability_score_base
                    dbg!(races.ability_score_increase(&race));
                    character.abilities.strength =
                        races.ability_score_increase(&race).abilities.strength;
                    character.abilities.dexterity =
                        races.ability_score_increase(&race).abilities.dexterity;
                    character.abilities.charisma =
                        races.ability_score_increase(&race).abilities.charisma;
                    character.abilities.constitution =
                        races.ability_score_increase(&race).abilities.constitution;
                    character.abilities.intellect =
                        races.ability_score_increase(&race).abilities.intellect;
                    character.abilities.wisdom =
                        races.ability_score_increase(&race).abilities.wisdom;

                    dbg!(&character);

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

        character
    }
}
