use crate::COLUMN_WIDTH;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use textwrap;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Class {
    class: String,
    description: String,
    hit_die: u8,
    primary_ability: Vec<String>,
    saving_throw_proficiencies: Vec<String>,
    armor_proficiencies: Vec<String>,
    weapon_proficiencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classes {
    classes: Vec<Class>,
}

impl Classes {
    pub fn new() -> Classes {
        let class_json = include_str!("classes.json");
        let classes: Vec<Class> = serde_json::from_str(&class_json).unwrap();
        Classes { classes }
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();
        for class in self.classes.iter() {
            keys.push(class.class.clone());
        }
        keys
    }

    fn value(&self, key: &str) -> Option<&Class> {
        for class in &self.classes {
            if class.class == key {
                return Some(class);
            }
        }
        None
    }

    pub fn class(&self, key: &str) -> String {
        self.value(key).unwrap().class.clone().to_case(Case::Title)
    }

    pub fn description(&self, key: &str) -> String {
        self.value(key).unwrap().description.clone()
    }

    pub fn hit_die(&self, key: &str) -> u8 {
        self.value(key).unwrap().hit_die
    }

    pub fn primary_ability(&self, key: &str) -> Vec<String> {
        self.value(key).unwrap().primary_ability.clone()
    }

    pub fn saving_throw_proficiencies(&self, key: &str) -> Vec<String> {
        self.value(key).unwrap().saving_throw_proficiencies.clone()
    }

    pub fn armor_proficiencies(&self, key: &str) -> Vec<String> {
        self.value(key).unwrap().armor_proficiencies.clone()
    }

    pub fn weapon_proficiencies(&self, key: &str) -> Vec<String> {
        self.value(key).unwrap().weapon_proficiencies.clone()
    }

    fn detail_class(&self, key: &str) -> String {
        format!("{}\n", self.class(key))
    }

    fn detail_description(&self, key: &str) -> String {
        format!(
            "{}\n",
            textwrap::fill(
                &self.description(key).to_case(Case::Title),
                textwrap::Options::new(COLUMN_WIDTH)
                    .initial_indent("  ")
                    .subsequent_indent("  ")
            )
        )
    }

    fn detail_hit_die(&self, key: &str) -> String {
        let hit_die = self.hit_die(key);
        let mut hit_die_str: String;

        hit_die_str = "- Hit Die\n".to_string();

        hit_die_str = format!("{}  - d{}\n", hit_die_str, hit_die);

        hit_die_str
    }

    fn detail_primary_ability(&self, key: &str) -> String {
        let primary_ability = self.primary_ability(key);
        let mut primary_ability_str: String;

        primary_ability_str = "- Primary Ability\n".to_string();

        if !primary_ability.is_empty() {
            primary_ability_str = format!(
                "{}{}\n",
                primary_ability_str,
                textwrap::fill(
                    &primary_ability.join(", ").to_case(Case::Title),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("    ")
                )
            );
        }

        primary_ability_str
    }

    fn detail_saving_throw_proficiencies(&self, key: &str) -> String {
        let saving_throw_proficiencies = self.saving_throw_proficiencies(key);
        let mut saving_throw_proficiencies_str: String;

        saving_throw_proficiencies_str = "- Saving Throw Proficiencies\n".to_string();

        if !saving_throw_proficiencies.is_empty() {
            saving_throw_proficiencies_str = format!(
                "{}{}\n",
                saving_throw_proficiencies_str,
                textwrap::fill(
                    &saving_throw_proficiencies.join(", ").to_case(Case::Title),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("    ")
                )
            );
        }

        saving_throw_proficiencies_str
    }

    fn detail_armor_proficiencies(&self, key: &str) -> String {
        let armor_proficiencies = self.armor_proficiencies(key);
        let mut armor_proficiencies_str: String;

        armor_proficiencies_str = "- Armor Proficiencies\n".to_string();

        if !armor_proficiencies.is_empty() {
            armor_proficiencies_str = format!(
                "{}{}\n",
                armor_proficiencies_str,
                textwrap::fill(
                    &armor_proficiencies.join(", ").to_case(Case::Title),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("    ")
                )
            );
        }

        armor_proficiencies_str
    }

    fn detail_weapon_proficiencies(&self, key: &str) -> String {
        let weapon_proficiencies = self.weapon_proficiencies(key);
        let mut weapon_proficiencies_str: String;

        weapon_proficiencies_str = "- Weapon Proficiencies\n".to_string();

        if !weapon_proficiencies.is_empty() {
            weapon_proficiencies_str = format!(
                "{}{}\n",
                weapon_proficiencies_str,
                textwrap::fill(
                    &weapon_proficiencies.join(", ").to_case(Case::Title),
                    textwrap::Options::new(COLUMN_WIDTH)
                        .initial_indent("  - ")
                        .subsequent_indent("    ")
                )
            );
        }

        weapon_proficiencies_str
    }

    pub fn details(&self, key: &str) -> String {
        [
            self.detail_class(key),
            self.detail_description(key),
            self.detail_hit_die(key),
            self.detail_primary_ability(key),
            self.detail_saving_throw_proficiencies(key),
            self.detail_armor_proficiencies(key),
            self.detail_weapon_proficiencies(key),
        ]
        .join("\n")
    }
}
