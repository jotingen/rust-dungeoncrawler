use crate::basics::{Abilities, Alignment};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Default)]
struct RaceAlignment {
    description: String,
    alignment: Alignment,
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
    nickname: Vec<String>,
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
    names: RaceNames,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Races {
    races: Vec<Race>,
}

impl Races {
    pub fn new() -> Races {
        let race_json = include_str!("races.json");
        let races: Vec<Race> = serde_json::from_str(&race_json).unwrap();
        Races { races }
    }
    pub fn print(&self) {
        println!("{:#?}", self)
    }
    pub fn list_races(&self) -> Vec<String> {
        let mut race_list: Vec<String> = Vec::new();
        for r in self.races.iter() {
            race_list.push(r.race.clone());
        }
        race_list
    }
    pub fn list_subraces(&self, race: &str) -> Vec<String> {
        let mut subrace_list: Vec<String> = Vec::new();
        for r in self.races.iter() {
            if r.race == *race {
                if !r.subraces.is_empty() {
                    for sr in r.subraces.iter() {
                        subrace_list.push(sr.sub_race.clone());
                    }
                }
            }
        }
        subrace_list
    }
}
