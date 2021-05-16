const COLUMN_WIDTH: usize = 80;

mod basics;
mod character;
mod races;

use crate::basics::{Abilities, Alignment};
use crate::character::Character;
use crate::races::Races;
use rand::Rng;
use regex::Regex;
use std::io::{stdin, stdout, Read, Write};

fn d(num: u8) -> u8 {
    if num == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    rng.gen_range(1..(num + 1))
}

fn pause() {
    let mut stdout = stdout();
    print!("Press Enter to continue...");
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

fn main() {
    //Load races
    let races: Races = Races::new();
    let mut character: Character;

    #[derive(Debug, PartialEq)]
    enum StateMain {
        Init,
        Load,
        CharacterCreation,
        Game,
        Exit,
    }

    let mut state_main = StateMain::Init;
    while state_main != StateMain::Exit {
        dbg!(&state_main);
        match state_main {
            StateMain::Init => {
                println!("Dungeon Crawler\n\n");
                state_main = StateMain::Load;
            }
            StateMain::Load => {
                println!("No savegame found, starting new game\n\n");
                state_main = StateMain::CharacterCreation;
            }
            StateMain::CharacterCreation => {
                character = character_creation(&races);
                state_main = StateMain::Game;
            }
            StateMain::Game => {
                state_main = StateMain::Exit;
            }
            StateMain::Exit => {
                state_main = StateMain::Exit;
            }
        }
    }
}

fn character_creation(races: &Races) -> Character {
    let mut character: Character = Character::new();
    let mut stats: [u8; 6];
    let mut race: String = "".to_string();
    let mut name: String = "".to_string();

    #[derive(Debug, PartialEq)]
    enum State {
        Init,
        Stats,
        Pick,
        Class,
        Name,
        Summary,
        Exit,
    }
    let mut state = State::Init;
    while state != State::Exit {
        dbg!(&state);
        match state {
            State::Init => {
                println!("Character Creation\n\n");
                state = State::Stats;
            }
            State::Stats => {
                println!("Stats\n");

                stats = [15, 14, 13, 12, 10, 8];
                println!("Default stats are {:?}", stats);

                if pick_yes_or_no("Roll your own stats?") {
                    roll_stats(&mut stats);
                }

                println!("Using stats {:?}", stats);

                pause();

                state = State::Pick;
            }
            State::Pick => {
                println!("Choose race:");

                for (count, r) in races.races().iter().enumerate() {
                    println!("{:>2}) {}", count + 1, races.race_type(&r));
                }

                let number = pick_number(
                    "Choose race, leave blank for random.",
                    1,
                    races.races().len() as u32,
                ) - 1;
                println!("{}", races.race_details(&races.races()[number as usize]));

                if pick_yes_or_no("Use this race?") {
                    race = races.races()[number as usize].to_string();
                    state = State::Class;
                } else {
                    state = State::Pick;
                }
            }
            State::Class => {
                pause();

                state = State::Name;
            }
            State::Name => {
                pause();

                println!("Choose name:");

                name = "Boaty McBoatface".to_string();

                state = State::Summary;
            }
            State::Summary => {
                character = Character::new();

                //name
                character.name = name.to_string();

                //race
                character.race = race.to_string();

                //age
                character.age = 25;

                //alignment
                character.alignment = Alignment::N;

                //ability_score_base
                dbg!(races.race_ability_score_increase(&race));
                character.abilities.strength =
                    races.race_ability_score_increase(&race).abilities.strength;
                character.abilities.dexterity =
                    races.race_ability_score_increase(&race).abilities.dexterity;
                character.abilities.charisma =
                    races.race_ability_score_increase(&race).abilities.charisma;
                character.abilities.constitution = races
                    .race_ability_score_increase(&race)
                    .abilities
                    .constitution;
                character.abilities.intellect =
                    races.race_ability_score_increase(&race).abilities.intellect;
                character.abilities.wisdom =
                    races.race_ability_score_increase(&race).abilities.wisdom;

                dbg!(&character);
                pause();

                state = State::Exit;
            }
            State::Exit => {
                state = State::Exit;
            }
        }
    }

    character
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

fn pick_yes_or_no(msg: &str) -> bool {
    println!("{} Y/n", msg);
    let mut my_yes_or_no_str = String::new();
    stdin().read_line(&mut my_yes_or_no_str).unwrap();

    //regex for empty/y*/Y*
    let re_yes = Regex::new(r"^(?i)\s*y(es)?\s*$").unwrap();
    if re_yes.is_match(&my_yes_or_no_str) || my_yes_or_no_str.trim().is_empty() {
        return true;
    }
    false
}

fn pick_number(msg: &str, low: u32, high: u32) -> u32 {
    loop {
        if !msg.is_empty() {
            print!("{} ", msg);
        }
        println!("{}-{}", low, high);
        let mut my_number_str = String::new();
        stdin().read_line(&mut my_number_str).unwrap();

        if my_number_str.trim().is_empty() {
            let mut rng = rand::thread_rng();
            return rng.gen_range(low..(high + 1));
        } else if my_number_str.trim().parse::<u32>().is_ok() {
            let my_number: u32 = my_number_str.trim().parse().unwrap();
            if my_number >= low && my_number <= high {
                return my_number;
            }
        }
    }
}
