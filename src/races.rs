use rand::Rng;

use crate::basics::{Abilities, Alignment, Gender};
use crate::screen::COLUMN_WIDTH;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RaceAbilities {
    description: String,
    pub abilities: Abilities,
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
    order: Vec<String>,
    child: Vec<String>,
    male: Vec<String>,
    female: Vec<String>,
    clan: Vec<String>,
    family: Vec<String>,
    surname: Vec<String>,
    nickname: Vec<String>,
    virtue: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RaceSpeed {
    description: String,
    speed: u32,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
pub struct RaceModifier {
    description: String,
    pub modifier: String,
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

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();
        for race in self.races.iter() {
            if race.subraces.is_empty() {
                keys.push(race.race.clone());
            } else {
                for subrace in race.subraces.iter() {
                    keys.push(subrace.race.clone());
                }
            }
        }
        keys
    }

    fn value(&self, key: &str) -> (Option<&Race>, Option<&SubRace>) {
        for race in &self.races {
            if race.race == key {
                return (Some(race), None);
            } else {
                for subrace in &race.subraces {
                    if subrace.race == key {
                        return (Some(race), Some(subrace));
                    }
                }
            }
        }

        (None, None)
    }

    pub fn race(&self, key: &str) -> String {
        let (race_opt, subrace_opt) = self.value(key);
        let race: &Race = race_opt.unwrap();
        let race_str: String;

        if let Some(subrace) = subrace_opt {
            race_str = subrace.race.clone();
        } else {
            race_str = race.race.clone();
        }

        race_str
    }

    pub fn description(&self, key: &str) -> String {
        let (race_opt, subrace_opt) = self.value(key);
        let race = race_opt.unwrap();
        let mut description_str: String;

        description_str = race.description.clone();
        if let Some(subrace) = subrace_opt {
            if !subrace.description.is_empty() {
                description_str = [description_str, subrace.description.clone()].join("\n");
            }
        }

        description_str
    }

    pub fn names(&self, key: &str) -> RaceNames {
        let (race_opt, subrace_opt) = self.value(key);
        let race = race_opt.unwrap();
        let mut names = RaceNames {
            ..Default::default()
        };

        names.description = race.names.description.clone();

        names.order = race.names.order.clone();

        names.child = race.names.child.clone();
        names.male = race.names.male.clone();
        names.female = race.names.female.clone();
        names.clan = race.names.clan.clone();
        names.family = race.names.family.clone();
        names.surname = race.names.surname.clone();
        names.nickname = race.names.nickname.clone();
        names.virtue = race.names.virtue.clone();

        if let Some(subrace) = subrace_opt {
            names.description = [names.description, subrace.names.description.clone()].join("\n");

            if !subrace.names.order.is_empty() {
                names.order = subrace.names.order.clone();
            }

            names.child.extend(subrace.names.child.clone());
            names.male.extend(subrace.names.male.clone());
            names.female.extend(subrace.names.female.clone());
            names.clan.extend(subrace.names.clan.clone());
            names.family.extend(subrace.names.family.clone());
            names.surname.extend(subrace.names.surname.clone());
            names.nickname.extend(subrace.names.nickname.clone());
            names.virtue.extend(subrace.names.virtue.clone());
        }

        names.child.sort();
        names.male.sort();
        names.female.sort();
        names.clan.sort();
        names.family.sort();
        names.surname.sort();
        names.nickname.sort();
        names.virtue.sort();

        names
    }

    pub fn generate_name(&self, key: &str, gender: Gender) -> String {
        let names = self.names(key);
        let mut name_str: String;

        name_str = "".to_string();

        let mut rng = rand::thread_rng();
        for name_type in names.order.iter() {
            if name_type == "childhood" && !names.child.is_empty() {
                name_str = format!(
                    "{} {}",
                    name_str,
                    names.child[rng.gen_range(0..names.child.len())]
                );
            }
            if name_type == "first" {
                if gender == Gender::M && !names.male.is_empty() {
                    name_str = format!(
                        "{} {}",
                        name_str,
                        names.male[rng.gen_range(0..names.male.len())]
                    );
                }
                if gender == Gender::F && !names.female.is_empty() {
                    name_str = format!(
                        "{} {}",
                        name_str,
                        names.female[rng.gen_range(0..names.female.len())]
                    );
                }
            }
            if name_type == "clan" && !names.clan.is_empty() {
                name_str = format!(
                    "{} {}",
                    name_str,
                    names.clan[rng.gen_range(0..names.clan.len())]
                );
            }
            if name_type == "family" && !names.family.is_empty() {
                name_str = format!(
                    "{} {}",
                    name_str,
                    names.family[rng.gen_range(0..names.family.len())]
                );
            }
            if name_type == "surname" && !names.surname.is_empty() {
                name_str = format!(
                    "{} {}",
                    name_str,
                    names.surname[rng.gen_range(0..names.surname.len())]
                );
            }
            if name_type == "nickname" && !names.nickname.is_empty() {
                name_str = format!(
                    "{} {}",
                    name_str,
                    names.nickname[rng.gen_range(0..names.nickname.len())]
                );
            }
            if name_type == "virtue" && !names.virtue.is_empty() {
                name_str = format!(
                    "{} {}",
                    name_str,
                    names.virtue[rng.gen_range(0..names.virtue.len())]
                );
            }
        }

        name_str.trim().to_string()
    }

    pub fn ability_score_increase(&self, key: &str) -> RaceAbilities {
        let (race_opt, subrace_opt) = self.value(key);
        let race = race_opt.unwrap();
        let mut ability_score_increase = race.ability_score_increase.clone();

        if let Some(subrace) = subrace_opt {
            ability_score_increase.description = [
                ability_score_increase.description,
                subrace.names.description.clone(),
            ]
            .join("\n");

            ability_score_increase.abilities.strength +=
                subrace.ability_score_increase.abilities.strength;
            ability_score_increase.abilities.dexterity +=
                subrace.ability_score_increase.abilities.dexterity;
            ability_score_increase.abilities.charisma +=
                subrace.ability_score_increase.abilities.charisma;
            ability_score_increase.abilities.constitution +=
                subrace.ability_score_increase.abilities.constitution;
            ability_score_increase.abilities.intellect +=
                subrace.ability_score_increase.abilities.intellect;
            ability_score_increase.abilities.wisdom +=
                subrace.ability_score_increase.abilities.wisdom;
        }

        ability_score_increase
    }

    pub fn age(&self, key: &str) -> RaceAge {
        let (race_opt, _) = self.value(key);
        let race = race_opt.unwrap();

        race.age.clone()
    }

    pub fn alignment(&self, key: &str) -> RaceAlignment {
        let (race_opt, _) = self.value(key);
        let race = race_opt.unwrap();

        race.alignment.clone()
    }

    pub fn size(&self, key: &str) -> RaceSize {
        let (race_opt, _) = self.value(key);
        let race = race_opt.unwrap();

        race.size.clone()
    }

    pub fn speed(&self, key: &str) -> RaceSpeed {
        let (race_opt, _) = self.value(key);
        let race = race_opt.unwrap();

        race.speed.clone()
    }

    pub fn modifiers(&self, key: &str) -> Vec<RaceModifier> {
        let (race_opt, subrace_opt) = self.value(key);
        let race = race_opt.unwrap();
        let mut modifiers: Vec<RaceModifier>;

        modifiers = race.modifiers.clone();
        if let Some(subrace) = subrace_opt {
            modifiers.extend(subrace.modifiers.clone());
        }
        modifiers.sort_by(|a, b| a.modifier.cmp(&b.modifier));

        modifiers
    }

    pub fn languages(&self, key: &str) -> Vec<String> {
        let (race_opt, _) = self.value(key);
        let race = race_opt.unwrap();
        let mut languages: Vec<String>;

        languages = race.languages.clone();
        languages.sort();

        languages
    }

    pub fn detail_race(&self, key: &str) -> String {
        let (race_opt, subrace_opt) = self.value(key);
        let race: &Race = race_opt.unwrap();
        let race_str: String;

        if let Some(subrace) = subrace_opt {
            race_str = format!(
                "{} ({})\n",
                subrace.race.clone().to_case(Case::Title),
                race.race.clone().to_case(Case::Title)
            )
        } else {
            race_str = format!("{}\n", race.race.clone().to_case(Case::Title))
        }

        race_str
    }

    fn detail_description(&self, key: &str) -> String {
        format!(
            "{}\n",
            textwrap::fill(
                &self.description(key),
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
        )
    }

    fn detail_names(&self, key: &str) -> String {
        let names = self.names(key);
        let mut names_str: String;

        names_str = "- Names\n".to_string();

        if !&names.description.trim().is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &names.description,
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  ")
                        .subsequent_indent("  ")
                )
            );
        }

        names_str = format!(
            "{}{}\n",
            names_str,
            textwrap::fill(
                &format!("Ordering: {}", names.order.join(" ").to_case(Case::Title)),
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
        );

        if !names.child.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("Childhood:      {}", names.child.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        if !names.male.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("First (Male):   {}", names.male.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        if !names.female.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("First (Female): {}", names.female.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        if !names.clan.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("Clan:           {}", names.clan.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        if !names.family.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("Family:         {}", names.family.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        if !names.surname.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("Surname:        {}", names.surname.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        if !names.nickname.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("Nicknames:      {}", names.nickname.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        if !names.virtue.is_empty() {
            names_str = format!(
                "{}{}\n",
                names_str,
                textwrap::fill(
                    &format!("Virtue:         {}", names.virtue.join(", ")),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("                    ")
                )
            );
        }

        names_str
    }

    fn detail_ability_score_increase(&self, key: &str) -> String {
        let ability_score_increase = self.ability_score_increase(key);
        let mut ability_score_increase_str: String;

        ability_score_increase_str = "- Ability Score Increase\n".to_string();

        ability_score_increase_str = format!(
            "{}{}\n",
            ability_score_increase_str,
            textwrap::fill(
                &ability_score_increase.description,
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
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

    fn detail_age(&self, key: &str) -> String {
        let age = self.age(key);
        let mut age_str: String;

        age_str = "- Age\n".to_string();

        age_str = format!(
            "{}{}\n",
            age_str,
            textwrap::fill(
                &age.description,
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
        );

        age_str = format!(
            "{}  - Adulthood: {}\n  - Lifespan: {}\n",
            age_str, age.adulthood, age.lifespan
        );

        age_str
    }

    fn detail_alignment(&self, key: &str) -> String {
        let alignment = self.alignment(key);
        let mut alignment_str: String;

        alignment_str = "- Alignment\n".to_string();

        alignment_str = format!(
            "{}{}\n",
            alignment_str,
            textwrap::fill(
                &alignment.description,
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
        );

        alignment_str = format!("{}  - {:?}\n", alignment_str, alignment.alignment);

        alignment_str
    }

    fn detail_size(&self, key: &str) -> String {
        let size = self.size(key);
        let mut size_str: String;

        size_str = "- Size\n".to_string();

        size_str = format!(
            "{}{}\n",
            size_str,
            textwrap::fill(
                &size.description,
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
        );

        size_str = format!(
            "{}  - {:?}: {}-{} ft\n",
            size_str, size.class, size.lower, size.upper
        );

        size_str
    }

    fn detail_speed(&self, key: &str) -> String {
        let speed = self.speed(key);
        let mut speed_str: String;

        speed_str = "- Speed\n".to_string();

        speed_str = format!(
            "{}{}\n",
            speed_str,
            textwrap::fill(
                &speed.description,
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
        );

        speed_str = format!("{}  - {} ft\n", speed_str, speed.speed);

        speed_str
    }

    fn detail_modifiers(&self, key: &str) -> String {
        let modifiers = self.modifiers(key);
        let mut modifiers_str: String;

        modifiers_str = "- Modifiers\n".to_string();

        for modifier in modifiers.iter() {
            modifiers_str = format!(
                "{}  - {}\n{}\n\n",
                modifiers_str,
                modifier.modifier.to_case(Case::Title),
                textwrap::fill(
                    &modifier.description,
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("    ")
                        .subsequent_indent("    ")
                )
            );
        }

        modifiers_str
    }

    fn detail_languages(&self, key: &str) -> String {
        let languages = self.languages(key);
        let mut languages_str: String;

        languages_str = "- Languages\n".to_string();
        if !languages.is_empty() {
            languages_str = format!(
                "{}{}\n",
                languages_str,
                textwrap::fill(
                    &languages.join(", ").to_case(Case::Title),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("    ")
                )
            );
        }

        languages_str
    }

    pub fn details(&self, key: &str) -> String {
        [
            self.detail_race(key),
            self.detail_description(key),
            self.detail_names(key),
            self.detail_ability_score_increase(key),
            self.detail_age(key),
            self.detail_alignment(key),
            self.detail_size(key),
            self.detail_speed(key),
            self.detail_modifiers(key),
            self.detail_languages(key),
        ]
        .join("\n")
    }
}
