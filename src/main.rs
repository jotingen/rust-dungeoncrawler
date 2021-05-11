mod basics;
mod races;

use crate::basics::{Abilities, Alignment};
use crate::races::Races;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};

fn main() {
    #[derive(Serialize, Deserialize, Debug, Default)]
    struct Character {
        name: String,
        //race: Race,
        age: u32,
        alignment: Alignment,
        ability_score_base: Abilities,
    }

    println!("Hello, world!");

    //Stats
    let mut rolls: [u8; 6] = [15, 14, 13, 12, 10, 8];
    println!("Default stats are {:?}", rolls);
    let mut roll_my_own = String::new();
    println!("Roll your own stats? Y/n");
    std::io::stdin().read_line(&mut roll_my_own).unwrap();
    let re_yes = Regex::new(r"^\s*[yY]\s*$").unwrap();
    if re_yes.is_match(&roll_my_own) {
        roll_stats(&mut rolls);
    }
    println!("Using stats {:?}", rolls);

    //Load races
    let races: Races = Races::new();
    //races.print();
    let race_list = races.list_races();
    for r in race_list.iter() {
        println!("{:#?}", r);
        for sr in races.list_subraces(r) {
            println!("  {:#?}", sr);
        }
    }

    //races::load();
    //races::print();
    //let race_json = include_str!("races.json");
    //let races: Vec<Race> = serde_json::from_str(&race_json).unwrap();
    //println!("{:#?}", races)
}

fn roll_stats(rolls: &mut [u8; 6]) {
    let mut rng = rand::thread_rng();
    println!("Rolling own");
    for roll in rolls.iter_mut() {
        let mut die_rolls: [u8; 4] = [
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
        ];
        die_rolls.sort_unstable();
        die_rolls.reverse();
        *roll = die_rolls[0] + die_rolls[1] + die_rolls[2];
        println!("Rolled {:?} for {}", die_rolls, *roll);
    }
    rolls.sort_unstable();
    rolls.reverse();
}
