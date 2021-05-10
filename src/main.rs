use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Abilities {
    strength: u32,
    dexterity: u32,
    constitution: u32,
    intellect: u32,
    wisdom: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceAbilities {
    description: String,
    abilities: Abilities,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceAge {
    description: String,
    adulthood: u32,
    lifespan: u32,
}

#[derive(Serialize, Deserialize, Debug)]
enum Alignment {
    LG, //Lawful good
    NG, //Neutral good
    CG, //Chaotic good
    LN, //Lawful neutral
    N,  //Neutral
    CN, //Chaotic neutral
    LE, //Lawful evil
    NE, //Neutral evil
    CE, //Chaotic evil
}
impl Default for Alignment {
    fn default() -> Self {
        Alignment::N
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceAlignment {
    description: String,
    alignent: Alignment,
}

#[derive(Serialize, Deserialize, Debug)]
enum SizeClass {
    Small,
    Medium,
    Large,
}
impl Default for SizeClass {
    fn default() -> Self {
        SizeClass::Medium
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceSize {
    description: String,
    lower: f32,
    upper: f32,
    class: SizeClass,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceNames {
    description: String,
    child: Vec<String>,
    male: Vec<String>,
    female: Vec<String>,
    clan: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceSpeed {
    description: String,
    speed: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceModifier {
    description: String,
    modifier: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SubRace {
    sub_race: String,
    description: String,
    ability_score_increase: Abilities,
    modifiers: Vec<RaceModifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Race {
    race: String,
    description: String,
    names: RaceNames,
    ability_score_increase: RaceAbilities,
    age: RaceAge,
    alignment: RaceAlignment,
    size: RaceSize,
    speed: RaceSpeed,
    modifiers: Vec<RaceModifier>,
    languages: Vec<String>,
    subraces: Vec<SubRace>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Character {
    name: String,
    race: Race,
    age: u32,
    alignment: Alignment,
    ability_score_base: Abilities,
}

fn main() {
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
    let race_json = include_str!("races.json");
    let races: Vec<Race> = serde_json::from_str(&race_json).unwrap();
    println!("{:#?}", races)
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
