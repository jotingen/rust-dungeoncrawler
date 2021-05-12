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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
struct RaceModifier {
    description: String,
    modifier: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SubRace {
    sub_race: String,
    description: String,
    names: RaceNames,
    ability_score_increase: RaceAbilities,
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
    pub fn details(&self, race_str: &str) -> String {
        let mut race: &Race = &Race {
            ..Default::default()
        };
        let mut subrace: &SubRace = &SubRace {
            ..Default::default()
        };
        let mut is_subrace: bool = false;
        let mut detail_str: String;

        //Find given race/subrace
        for r in self.races.iter() {
            if r.race == race_str {
                race = r;
            } else {
                for sr in r.subraces.iter() {
                    if sr.sub_race == race_str {
                        race = r;
                        subrace = sr;
                        is_subrace = true;
                    }
                }
            }
        }

        //Race
        if is_subrace {
            detail_str = format!("{}\n", subrace.sub_race)
        } else {
            detail_str = format!("{}\n", race.race)
        }

        //Description
        if is_subrace {
            detail_str = format!(
                "{}\n- {} ({})\n{}\n{}\n",
                detail_str, subrace.sub_race, race.race, race.description, subrace.description
            );
        } else {
            detail_str = format!("{}\n- {}\n{}\n", detail_str, race.race, race.description);
        }

        //names
        detail_str = format!("{}\n- Names\n", detail_str);
        if !race.names.description.is_empty() {
            detail_str = format!("{}{}\n", detail_str, race.names.description)
        }
        if is_subrace && !subrace.names.description.is_empty() {
            detail_str = format!("{}{}\n", detail_str, subrace.names.description)
        }

        let mut names_child: Vec<String> = Vec::new();
        names_child = race.names.child.clone();
        if is_subrace {
            names_child.extend(subrace.names.child.clone());
        }
        if !names_child.is_empty() {
            names_child.sort();
            detail_str = format!("{}  - Childhood: {}\n", detail_str, names_child.join(", "));
        }

        let mut names_male: Vec<String> = Vec::new();
        names_male = race.names.male.clone();
        if is_subrace {
            names_male.extend(subrace.names.male.clone());
        }
        if !names_male.is_empty() {
            names_male.sort();
            detail_str = format!(
                "{}  - First (Male): {}\n",
                detail_str,
                names_male.join(", ")
            );
        }

        let mut names_female: Vec<String> = Vec::new();
        names_female = race.names.female.clone();
        if is_subrace {
            names_female.extend(subrace.names.female.clone());
        }
        if !names_female.is_empty() {
            names_female.sort();
            detail_str = format!(
                "{}  - First (Female): {}\n",
                detail_str,
                names_female.join(", ")
            );
        }

        let mut names_clan: Vec<String> = Vec::new();
        names_clan = race.names.clan.clone();
        if is_subrace {
            names_clan.extend(subrace.names.clan.clone());
        }
        if !names_clan.is_empty() {
            names_clan.sort();
            detail_str = format!("{}  - Clan: {}\n", detail_str, names_clan.join(", "));
        }

        let mut names_nickname: Vec<String> = Vec::new();
        names_nickname = race.names.nickname.clone();
        if is_subrace {
            names_nickname.extend(subrace.names.nickname.clone());
        }
        if !names_nickname.is_empty() {
            names_nickname.sort();
            detail_str = format!(
                "{}  - Nicknames: {}\n",
                detail_str,
                names_nickname.join(", ")
            );
        }

        //ability_score_increase
        detail_str = format!("{}\n- Ability Score Increase\n", detail_str);
        if !race.ability_score_increase.description.is_empty() {
            detail_str = format!(
                "{}{}\n",
                detail_str, race.ability_score_increase.description
            )
        }
        if is_subrace && !subrace.ability_score_increase.description.is_empty() {
            detail_str = format!(
                "{}{}\n",
                detail_str, subrace.ability_score_increase.description
            )
        }
        detail_str = format!(
            "{}  - Str:{} Dex:{} Cha:{} Con:{} Int:{} Wis:{}\n",
            detail_str,
            race.ability_score_increase.abilities.strength
                + subrace.ability_score_increase.abilities.strength,
            race.ability_score_increase.abilities.dexterity
                + subrace.ability_score_increase.abilities.dexterity,
            race.ability_score_increase.abilities.charisma
                + subrace.ability_score_increase.abilities.charisma,
            race.ability_score_increase.abilities.constitution
                + subrace.ability_score_increase.abilities.constitution,
            race.ability_score_increase.abilities.intellect
                + subrace.ability_score_increase.abilities.intellect,
            race.ability_score_increase.abilities.wisdom
                + subrace.ability_score_increase.abilities.wisdom
        );

        //age
        detail_str = format!("{}\n- Age\n", detail_str);
        if !race.age.description.is_empty() {
            detail_str = format!("{}{}\n", detail_str, race.age.description)
        }
        detail_str = format!(
            "{}  - Adulthood: {}\n  - Lifespan: {}\n",
            detail_str, race.age.adulthood, race.age.lifespan
        );

        //alignment
        detail_str = format!("{}\n- Alignment\n", detail_str);
        if !race.alignment.description.is_empty() {
            detail_str = format!("{}{}\n", detail_str, race.alignment.description)
        }
        detail_str = format!("{}  - {:?}\n", detail_str, race.alignment.alignment);

        //size
        detail_str = format!("{}\n- Size\n", detail_str);
        if !race.size.description.is_empty() {
            detail_str = format!("{}{}\n", detail_str, race.size.description)
        }
        detail_str = format!(
            "{}  - {:?}: {}-{} ft\n",
            detail_str, race.size.class, race.size.lower, race.size.upper
        );

        //speed
        detail_str = format!("{}\n- Speed\n", detail_str);
        if !race.speed.description.is_empty() {
            detail_str = format!("{}{}\n", detail_str, race.speed.description)
        }
        detail_str = format!("{}  - {} ft\n", detail_str, race.speed.speed);

        //modifiers
        detail_str = format!("{}\n- Modifiers\n", detail_str);

        let mut modifiers: Vec<RaceModifier> = Vec::new();
        modifiers = race.modifiers.clone();
        if is_subrace {
            modifiers.extend(subrace.modifiers.clone());
        }
        modifiers.sort_by(|a, b| a.modifier.cmp(&b.modifier));
        for modifier in modifiers.iter() {
            detail_str = format!(
                "{}  - {}\n    {}\n",
                detail_str, modifier.modifier, modifier.description
            );
        }

        //languages
        detail_str = format!("{}\n- Languages\n", detail_str);
        if !race.languages.is_empty() {
            detail_str = format!("{}  - {}\n", detail_str, race.languages.join(", "));
        }

        detail_str
    }
}
