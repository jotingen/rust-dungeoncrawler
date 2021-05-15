use crate::basics::{Abilities, Alignment};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RaceAbilities {
    description: String,
    abilities: Abilities,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RaceAge {
    description: String,
    adulthood: u32,
    lifespan: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RaceAlignment {
    description: String,
    alignment: Alignment,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SizeClass {
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
pub struct RaceSize {
    description: String,
    lower: f32,
    upper: f32,
    class: SizeClass,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RaceNames {
    description: String,
    child: Vec<String>,
    male: Vec<String>,
    female: Vec<String>,
    clan: Vec<String>,
    nickname: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RaceSpeed {
    description: String,
    speed: u32,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
pub struct RaceModifier {
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

    pub fn race_description(&self, race_str: &str) -> String {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut description_str: String;

        description_str = format!("{}", race.description);
        if !subrace.description.is_empty() {
            description_str = format!("{}\n{}", description_str, subrace.description)
        }

        description_str
    }

    pub fn race_names(&self, race_str: &str) -> RaceNames {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut names = RaceNames {
            ..Default::default()
        };

        names.description = [race.names.description, subrace.names.description].join("\n");

        names.child = race.names.child;
        names.child.extend(subrace.names.child);
        names.child.sort();

        names.male = race.names.male;
        names.male.extend(subrace.names.male);
        names.male.sort();

        names.female = race.names.female;
        names.female.extend(subrace.names.female);
        names.female.sort();

        names.clan = race.names.clan;
        names.clan.extend(subrace.names.clan);
        names.clan.sort();

        names.nickname = race.names.nickname;
        names.nickname.extend(subrace.names.nickname);
        names.nickname.sort();

        names
    }

    pub fn race_ability_score_increase(&self, race_str: &str) -> RaceAbilities {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut ability_score_increase = RaceAbilities {
            ..Default::default()
        };

        ability_score_increase.description = [
            race.ability_score_increase.description,
            subrace.ability_score_increase.description,
        ]
        .join("\n");

        ability_score_increase.abilities.strength = race.ability_score_increase.abilities.strength
            + subrace.ability_score_increase.abilities.strength;
        ability_score_increase.abilities.dexterity =
            race.ability_score_increase.abilities.dexterity
                + subrace.ability_score_increase.abilities.dexterity;
        ability_score_increase.abilities.charisma = race.ability_score_increase.abilities.charisma
            + subrace.ability_score_increase.abilities.charisma;
        ability_score_increase.abilities.constitution =
            race.ability_score_increase.abilities.constitution
                + subrace.ability_score_increase.abilities.constitution;
        ability_score_increase.abilities.intellect =
            race.ability_score_increase.abilities.intellect
                + subrace.ability_score_increase.abilities.intellect;
        ability_score_increase.abilities.wisdom = race.ability_score_increase.abilities.wisdom
            + subrace.ability_score_increase.abilities.wisdom;

        ability_score_increase
    }

    pub fn race_age(&self, race_str: &str) -> RaceAge {
        let (race, _subrace) = self.get_race_structs(race_str);

        race.age
    }

    pub fn race_alignment(&self, race_str: &str) -> RaceAlignment {
        let (race, _subrace) = self.get_race_structs(race_str);

        race.alignment
    }

    pub fn race_size(&self, race_str: &str) -> RaceSize {
        let (race, _subrace) = self.get_race_structs(race_str);

        race.size
    }

    pub fn race_speed(&self, race_str: &str) -> RaceSpeed {
        let (race, _subrace) = self.get_race_structs(race_str);

        race.speed
    }

    pub fn race_modifiers(&self, race_str: &str) -> Vec<RaceModifier> {
        let (race, subrace) = self.get_race_structs(race_str);
        let mut modifiers: Vec<RaceModifier>;

        modifiers = race.modifiers;
        modifiers.extend(subrace.modifiers);
        modifiers.sort_by(|a, b| a.modifier.cmp(&b.modifier));

        modifiers
    }

    pub fn race_languages(&self, race_str: &str) -> Vec<String> {
        let (race, _subrace) = self.get_race_structs(race_str);
        let mut languages: Vec<String>;

        languages = race.languages;
        languages.sort();

        languages
    }

    fn race_detail_type(&self, race_str: &str) -> String {
        format!("{}\n", self.race_type(race_str))
    }

    fn race_detail_description(&self, race_str: &str) -> String {
        format!("{}\n", self.race_description(race_str))
    }

    fn race_detail_names(&self, race_str: &str) -> String {
        let names = self.race_names(race_str);
        let mut names_str: String;

        names_str = "- Names\n".to_string();
        names_str = format!("{}  {}\n", names_str, names.description);

        if !names.child.is_empty() {
            names_str = format!("{}  - Childhood: {}\n", names_str, names.child.join(", "));
        }

        if !names.male.is_empty() {
            names_str = format!("{}  - First (Male): {}\n", names_str, names.male.join(", "));
        }

        if !names.female.is_empty() {
            names_str = format!(
                "{}  - First (Female): {}\n",
                names_str,
                names.female.join(", ")
            );
        }

        if !names.clan.is_empty() {
            names_str = format!("{}  - Clan: {}\n", names_str, names.clan.join(", "));
        }

        if !names.nickname.is_empty() {
            names_str = format!(
                "{}  - Nicknames: {}\n",
                names_str,
                names.nickname.join(", ")
            );
        }

        names_str
    }

    fn race_detail_ability_score_increase(&self, race_str: &str) -> String {
        let ability_score_increase = self.race_ability_score_increase(race_str);
        let mut ability_score_increase_str: String;

        ability_score_increase_str = "- Ability Score Increase\n".to_string();
        ability_score_increase_str = format!(
            "{}  {}\n",
            ability_score_increase_str, ability_score_increase.description
        );

        ability_score_increase_str = format!(
            "{}  - Str:{} Dex:{} Cha:{} Con:{} Int:{} Wis:{}\n",
            ability_score_increase_str,
            ability_score_increase.abilities.strength,
            ability_score_increase.abilities.dexterity,
            ability_score_increase.abilities.charisma,
            ability_score_increase.abilities.constitution,
            ability_score_increase.abilities.intellect,
            ability_score_increase.abilities.wisdom
        );

        ability_score_increase_str
    }

    fn race_detail_age(&self, race_str: &str) -> String {
        let age = self.race_age(race_str);
        let mut age_str: String;

        age_str = "- Age\n".to_string();
        if !age.description.is_empty() {
            age_str = format!("{}  {}\n", age_str, age.description)
        }
        age_str = format!(
            "{}  - Adulthood: {}\n  - Lifespan: {}\n",
            age_str, age.adulthood, age.lifespan
        );

        age_str
    }

    fn race_detail_alignment(&self, race_str: &str) -> String {
        let alignment = self.race_alignment(race_str);
        let mut alignment_str: String;

        alignment_str = "- Alignment\n".to_string();
        if !alignment.description.is_empty() {
            alignment_str = format!("{}  {}\n", alignment_str, alignment.description)
        }
        alignment_str = format!("{}  - {:?}\n", alignment_str, alignment.alignment);

        alignment_str
    }

    fn race_detail_size(&self, race_str: &str) -> String {
        let size = self.race_size(race_str);
        let mut size_str: String;

        size_str = "- Size\n".to_string();
        if !size.description.is_empty() {
            size_str = format!("{}  {}\n", size_str, size.description)
        }
        size_str = format!(
            "{}  - {:?}: {}-{} ft\n",
            size_str, size.class, size.lower, size.upper
        );

        size_str
    }

    fn race_detail_speed(&self, race_str: &str) -> String {
        let speed = self.race_speed(race_str);
        let mut speed_str: String;

        speed_str = "- Speed\n".to_string();
        if !speed.description.is_empty() {
            speed_str = format!("{}  {}\n", speed_str, speed.description)
        }
        speed_str = format!("{}  - {} ft\n", speed_str, speed.speed);

        speed_str
    }

    fn race_detail_modifiers(&self, race_str: &str) -> String {
        let modifiers = self.race_modifiers(race_str);
        let mut modifiers_str: String;

        modifiers_str = "- Modifiers\n".to_string();

        for modifier in modifiers.iter() {
            modifiers_str = format!(
                "{}  - {}\n    {}\n",
                modifiers_str, modifier.modifier, modifier.description
            );
        }

        modifiers_str
    }

    fn race_detail_languages(&self, race_str: &str) -> String {
        let languages = self.race_languages(race_str);
        let mut languages_str: String;

        languages_str = "- Languages\n".to_string();
        if !languages.is_empty() {
            languages_str = format!("{}  - {}\n", languages_str, languages.join(", "));
        }

        languages_str
    }

    pub fn race_details(&self, race_str: &str) -> String {
        [
            self.race_detail_type(race_str),
            self.race_detail_description(race_str),
            self.race_detail_names(race_str),
            self.race_detail_ability_score_increase(race_str),
            self.race_detail_age(race_str),
            self.race_detail_alignment(race_str),
            self.race_detail_size(race_str),
            self.race_detail_speed(race_str),
            self.race_detail_modifiers(race_str),
            self.race_detail_languages(race_str),
        ]
        .join("\n")
    }
}
