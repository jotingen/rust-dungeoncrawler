use crate::basics::{Abilities, Alignment};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct RaceAbilities {
    description: String,
    abilities: Abilities,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct RaceAge {
    description: String,
    adulthood: u32,
    lifespan: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct RaceAlignment {
    description: String,
    alignment: Alignment,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct RaceSize {
    description: String,
    lower: f32,
    upper: f32,
    class: SizeClass,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct RaceNames {
    description: String,
    child: Vec<String>,
    male: Vec<String>,
    female: Vec<String>,
    clan: Vec<String>,
    nickname: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct RaceSpeed {
    description: String,
    speed: u32,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
struct RaceModifier {
    description: String,
    modifier: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct SubRace {
    race: String,
    description: String,
    names: RaceNames,
    ability_score_increase: RaceAbilities,
    modifiers: Vec<RaceModifier>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

//Each race mayhave subraces
//If a race has a subrace, only consider those subraces as races
impl Races {
    pub fn new() -> Races {
        let race_json = include_str!("races.json");
        let races: Vec<Race> = serde_json::from_str(&race_json).unwrap();
        Races { races }
    }

    pub fn print(&self) {
        println!("{:#?}", self)
    }

    pub fn races(&self) -> Vec<String> {
        let mut race_list: Vec<String> = Vec::new();
        for race in self.races.iter() {
            if race.subraces.is_empty() {
                race_list.push(race.race.clone());
            } else {
                for subrace in race.subraces.iter() {
                    race_list.push(subrace.race.clone());
                }
            }
        }
        race_list
    }

    fn get_race_structs(&self, race_str: &str) -> (Race, SubRace) {
        //Find given race/subrace
        let mut race: Race = Race {
            ..Default::default()
        };
        let mut subrace: SubRace = SubRace {
            ..Default::default()
        };

        for r in self.races.iter() {
            if r.race == race_str {
                race = r.clone();
            } else {
                for sr in r.subraces.iter() {
                    if sr.race == race_str {
                        race = r.clone();
                        subrace = sr.clone();
                    }
                }
            }
        }

        (race, subrace)
    }

    pub fn race_type(&self, race_str: &str) -> String {
        let (race, subrace) = self.get_race_structs(race_str);
        let type_str: String;

        if subrace.race.is_empty() {
            type_str = format!("{}", race.race)
        } else {
            type_str = format!("{} ({})", subrace.race, race.race)
        }

        type_str
    }

    fn race_detail_type(&self, race_str: &str) -> String {
        format!("{}\n", self.race_type(race_str))
    }

    fn race_detail_description(&self, race_str: &str) -> String {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut description_str: String;

        description_str = format!("{}\n", race.description);
        if !subrace.description.is_empty() {
            description_str = format!("{}{}\n", description_str, subrace.description)
        }

        description_str
    }

    fn race_detail_names(&self, race_str: &str) -> String {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut names_str: String;

        names_str = "- Names\n".to_string();
        if !race.names.description.is_empty() {
            names_str = format!("{}{}\n", names_str, race.names.description)
        }
        if !subrace.names.description.is_empty() {
            names_str = format!("{}{}\n", names_str, subrace.names.description)
        }

        let mut names_child: Vec<String>;
        names_child = race.names.child;
        names_child.extend(subrace.names.child);
        if !names_child.is_empty() {
            names_child.sort();
            names_str = format!("{}  - Childhood: {}\n", names_str, names_child.join(", "));
        }

        let mut names_male: Vec<String>;
        names_male = race.names.male;
        names_male.extend(subrace.names.male);
        if !names_male.is_empty() {
            names_male.sort();
            names_str = format!("{}  - First (Male): {}\n", names_str, names_male.join(", "));
        }

        let mut names_female: Vec<String>;
        names_female = race.names.female;
        names_female.extend(subrace.names.female);
        if !names_female.is_empty() {
            names_female.sort();
            names_str = format!(
                "{}  - First (Female): {}\n",
                names_str,
                names_female.join(", ")
            );
        }

        let mut names_clan: Vec<String>;
        names_clan = race.names.clan;
        names_clan.extend(subrace.names.clan);
        if !names_clan.is_empty() {
            names_clan.sort();
            names_str = format!("{}  - Clan: {}\n", names_str, names_clan.join(", "));
        }

        let mut names_nickname: Vec<String>;
        names_nickname = race.names.nickname;
        names_nickname.extend(subrace.names.nickname);
        if !names_nickname.is_empty() {
            names_nickname.sort();
            names_str = format!(
                "{}  - Nicknames: {}\n",
                names_str,
                names_nickname.join(", ")
            );
        }

        names_str
    }

    fn race_detail_ability_score_increase(&self, race_str: &str) -> String {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut ability_score_increase_str: String;

        ability_score_increase_str = "- Ability Score Increase\n".to_string();
        if !race.ability_score_increase.description.is_empty() {
            ability_score_increase_str = format!(
                "{}{}\n",
                ability_score_increase_str, race.ability_score_increase.description
            )
        }
        if !subrace.ability_score_increase.description.is_empty() {
            ability_score_increase_str = format!(
                "{}{}\n",
                ability_score_increase_str, subrace.ability_score_increase.description
            )
        }
        ability_score_increase_str = format!(
            "{}  - Str:{} Dex:{} Cha:{} Con:{} Int:{} Wis:{}\n",
            ability_score_increase_str,
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

        ability_score_increase_str
    }

    fn race_detail_age(&self, race_str: &str) -> String {
        let (race, _subrace) = self.get_race_structs(race_str);
        let mut age_str: String;

        age_str = "- Age\n".to_string();
        if !race.age.description.is_empty() {
            age_str = format!("{}{}\n", age_str, race.age.description)
        }
        age_str = format!(
            "{}  - Adulthood: {}\n  - Lifespan: {}\n",
            age_str, race.age.adulthood, race.age.lifespan
        );

        age_str
    }

    fn race_detail_alignment(&self, race_str: &str) -> String {
        let (race, _subrace) = self.get_race_structs(race_str);
        let mut alignment_str: String;

        alignment_str = "- Alignment\n".to_string();
        if !race.alignment.description.is_empty() {
            alignment_str = format!("{}{}\n", alignment_str, race.alignment.description)
        }
        alignment_str = format!("{}  - {:?}\n", alignment_str, race.alignment.alignment);

        alignment_str
    }

    fn race_detail_size(&self, race_str: &str) -> String {
        let (race, _subrace) = self.get_race_structs(race_str);
        let mut size_str: String;

        size_str = "- Size\n".to_string();
        if !race.size.description.is_empty() {
            size_str = format!("{}{}\n", size_str, race.size.description)
        }
        size_str = format!(
            "{}  - {:?}: {}-{} ft\n",
            size_str, race.size.class, race.size.lower, race.size.upper
        );

        size_str
    }

    fn race_detail_speed(&self, race_str: &str) -> String {
        let (race, _subrace) = self.get_race_structs(race_str);
        let mut speed_str: String;

        speed_str = "- Speed\n".to_string();
        if !race.speed.description.is_empty() {
            speed_str = format!("{}{}\n", speed_str, race.speed.description)
        }
        speed_str = format!("{}  - {} ft\n", speed_str, race.speed.speed);

        speed_str
    }

    fn race_detail_modifiers(&self, race_str: &str) -> String {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut modifiers_str: String;

        modifiers_str = "- Modifiers\n".to_string();

        let mut modifiers: Vec<RaceModifier>;
        modifiers = race.modifiers;
        modifiers.extend(subrace.modifiers);
        modifiers.sort_by(|a, b| a.modifier.cmp(&b.modifier));
        for modifier in modifiers.iter() {
            modifiers_str = format!(
                "{}  - {}\n    {}\n",
                modifiers_str, modifier.modifier, modifier.description
            );
        }

        modifiers_str
    }

    fn race_detail_languages(&self, race_str: &str) -> String {
        let (race, _subrace) = self.get_race_structs(race_str);
        let mut languages_str: String;

        languages_str = "- Languages\n".to_string();
        if !race.languages.is_empty() {
            languages_str = format!("{}  - {}\n", languages_str, race.languages.join(", "));
        }

        languages_str
    }

    pub fn race_details(&self, race_str: &str) -> String {
        let mut detail_str: String;

        //Race
        detail_str = format!("{}\n", &self.race_detail_type(race_str));

        //Description
        detail_str = format!(
            "{}{}\n",
            detail_str,
            &self.race_detail_description(race_str)
        );

        //names
        detail_str = format!("{}{}\n", detail_str, &self.race_detail_names(race_str));

        //ability_score_increase
        detail_str = format!(
            "{}{}\n",
            detail_str,
            &self.race_detail_ability_score_increase(race_str)
        );

        //age
        detail_str = format!("{}{}\n", detail_str, &self.race_detail_age(race_str));

        //alignment
        detail_str = format!("{}{}\n", detail_str, &self.race_detail_alignment(race_str));

        //size
        detail_str = format!("{}{}\n", detail_str, &self.race_detail_size(race_str));

        //speed
        detail_str = format!("{}{}\n", detail_str, &self.race_detail_speed(race_str));

        //modifiers
        detail_str = format!("{}{}\n", detail_str, &self.race_detail_modifiers(race_str));

        //languages
        detail_str = format!("{}{}\n", detail_str, &self.race_detail_languages(race_str));

        detail_str
    }
}
