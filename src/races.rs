use crate::basics::{Abilities, Alignment};
use crate::COLUMN_WIDTH;
use serde::{Deserialize, Serialize};
use textwrap;

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

    fn structs(&self, key: &str) -> (Race, SubRace) {
        //Find given race/subrace
        let mut race: Race = Race {
            ..Default::default()
        };
        let mut subrace: SubRace = SubRace {
            ..Default::default()
        };

        for r in self.races.iter() {
            if r.race == key {
                race = r.clone();
            } else {
                for sr in r.subraces.iter() {
                    if sr.race == key {
                        race = r.clone();
                        subrace = sr.clone();
                    }
                }
            }
        }

        (race, subrace)
    }

    pub fn race(&self, key: &str) -> String {
        let (race, subrace) = self.structs(key);
        let race_str: String;

        if subrace.race.is_empty() {
            race_str = race.race
        } else {
            race_str = format!("{} ({})", subrace.race, race.race)
        }

        race_str
    }

    pub fn description(&self, key: &str) -> String {
        let (race, subrace) = self.structs(key);
        let mut description_str: String;

        description_str = race.description;
        if !subrace.description.is_empty() {
            description_str = format!("{}\n{}", description_str, subrace.description)
        }

        description_str
    }

    pub fn names(&self, key: &str) -> RaceNames {
        let (race, subrace) = self.structs(key);
        let mut names = RaceNames {
            ..Default::default()
        };

        names.description = [race.names.description, subrace.names.description].join("\n");

        names.order = race.names.order;
        if !subrace.names.order.is_empty() {
            names.order = subrace.names.order;
        }

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

        names.family = race.names.family;
        names.family.extend(subrace.names.family);
        names.family.sort();

        names.surname = race.names.surname;
        names.surname.extend(subrace.names.surname);
        names.surname.sort();

        names.nickname = race.names.nickname;
        names.nickname.extend(subrace.names.nickname);
        names.nickname.sort();

        names.virtue = race.names.virtue;
        names.virtue.extend(subrace.names.virtue);
        names.virtue.sort();

        names
    }

    pub fn ability_score_increase(&self, key: &str) -> RaceAbilities {
        let (race, subrace) = self.structs(key);
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

    pub fn age(&self, key: &str) -> RaceAge {
        let (race, _subrace) = self.structs(key);

        race.age
    }

    pub fn alignment(&self, key: &str) -> RaceAlignment {
        let (race, _subrace) = self.structs(key);

        race.alignment
    }

    pub fn size(&self, key: &str) -> RaceSize {
        let (race, _subrace) = self.structs(key);

        race.size
    }

    pub fn speed(&self, key: &str) -> RaceSpeed {
        let (race, _subrace) = self.structs(key);

        race.speed
    }

    pub fn modifiers(&self, key: &str) -> Vec<RaceModifier> {
        let (race, subrace) = self.structs(key);
        let mut modifiers: Vec<RaceModifier>;

        modifiers = race.modifiers;
        modifiers.extend(subrace.modifiers);
        modifiers.sort_by(|a, b| a.modifier.cmp(&b.modifier));

        modifiers
    }

    pub fn languages(&self, key: &str) -> Vec<String> {
        let (race, _subrace) = self.structs(key);
        let mut languages: Vec<String>;

        languages = race.languages;
        languages.sort();

        languages
    }

    fn detail_race(&self, key: &str) -> String {
        format!("{}\n", self.race(key))
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

        let mut order_uppercase = names.order;
        for order_uppercase_name in &mut order_uppercase {
            if let Some(letter) = order_uppercase_name.get_mut(0..1) {
                letter.make_ascii_uppercase();
            }
        }
        names_str = format!(
            "{}{}\n",
            names_str,
            textwrap::fill(
                &format!("Ordering: {}", order_uppercase.join(" ")),
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
                modifier.modifier,
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
                    &languages.join(", "),
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
