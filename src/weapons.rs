use crate::COLUMN_WIDTH;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use textwrap;

#[derive(Debug, Default, Serialize, Deserialize)]
struct WeaponProperties {
    finesse: bool,
    heavy: bool,
    light: bool,
    loading: bool,
    ammunition: bool,
    range_normal: u32,
    range_max: u32,
    reach: bool,
    special: bool,
    thrown: bool,
    two_handed: bool,
    versatile: bool,
    versatile_dmg: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Weapon {
    weapon: String,
    proficiency: String,
    class: String,
    cost: u32,
    damage: String,
    weight: f32,
    properties: WeaponProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapons {
    weapons: Vec<Weapon>,
}

impl Weapons {
    pub fn new() -> Weapons {
        let weapon_json = include_str!("weapons.json");
        let weapons: Vec<Weapon> = serde_json::from_str(&weapon_json).unwrap();
        Weapons { weapons }
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();
        for weapon in self.weapons.iter() {
            keys.push(weapon.weapon.clone());
        }
        keys
    }

    fn value(&self, key: &str) -> Option<&Weapon> {
        for weapon in &self.weapons {
            if weapon.weapon == key {
                return Some(weapon);
            }
        }
        None
    }

    pub fn weapon(&self, key: &str) -> String {
        self.value(key).unwrap().weapon.clone()
    }

    pub fn proficiency(&self, key: &str) -> String {
        self.value(key).unwrap().proficiency.clone()
    }

    pub fn class(&self, key: &str) -> String {
        self.value(key).unwrap().class.clone()
    }

    pub fn cost(&self, key: &str) -> u32 {
        self.value(key).unwrap().cost
    }

    pub fn damage(&self, key: &str) -> String {
        self.value(key).unwrap().damage.clone()
    }

    pub fn weight(&self, key: &str) -> f32 {
        self.value(key).unwrap().weight
    }

    pub fn finesse(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.finesse
    }

    pub fn heavy(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.heavy
    }

    pub fn light(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.light
    }

    pub fn loading(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.loading
    }

    pub fn ammunition(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.ammunition
    }

    pub fn range_normal(&self, key: &str) -> u32 {
        self.value(key).unwrap().properties.range_normal
    }

    pub fn range_max(&self, key: &str) -> u32 {
        self.value(key).unwrap().properties.range_max
    }

    pub fn reach(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.reach
    }

    pub fn special(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.special
    }

    pub fn thrown(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.thrown
    }

    pub fn two_handed(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.two_handed
    }

    pub fn versatile(&self, key: &str) -> bool {
        self.value(key).unwrap().properties.versatile
    }

    pub fn versatile_dmg(&self, key: &str) -> String {
        self.value(key).unwrap().properties.versatile_dmg.clone()
    }

    pub fn detail_weapon(&self, key: &str) -> String {
        format!("{}\n", self.weapon(key).to_case(Case::Title))
    }

    fn detail_proficiency(&self, key: &str) -> String {
        let proficiency = self.proficiency(key);
        let mut proficiency_str: String;

        proficiency_str = "- Proficiency:".to_string();

        proficiency_str = format!("{} {}\n", proficiency_str, proficiency.to_case(Case::Title));

        proficiency_str
    }

    fn detail_class(&self, key: &str) -> String {
        let class = self.class(key);
        let mut class_str: String;

        class_str = "- Class:".to_string();

        class_str = format!("{} {}\n", class_str, class.to_case(Case::Title));

        class_str
    }

    fn detail_cost(&self, key: &str) -> String {
        let cost = self.cost(key);
        let mut cost_str: String;

        cost_str = "- Cost:".to_string();

        cost_str = format!("{} {} cp\n", cost_str, cost);

        cost_str
    }

    fn detail_damage(&self, key: &str) -> String {
        let damage = self.damage(key);
        let mut damage_str: String;

        damage_str = "- Damage:".to_string();

        damage_str = format!("{} {}\n", damage_str, damage.to_case(Case::Title));

        damage_str
    }

    fn detail_weight(&self, key: &str) -> String {
        let weight = self.weight(key);
        let mut weight_str: String;

        weight_str = "- Weight:".to_string();

        weight_str = format!("{} {} lb\n", weight_str, weight);

        weight_str
    }

    fn detail_properties(&self, key: &str) -> String {
        let mut properties_str: String;

        properties_str = "- Properties:".to_string();

        let mut properties_vec: Vec<String> = Vec::new();

        if self.finesse(key) {
            properties_vec.push("finesse".to_string());
        }
        if self.heavy(key) {
            properties_vec.push("heavy".to_string());
        }
        if self.light(key) {
            properties_vec.push("light".to_string());
        }
        if self.loading(key) {
            properties_vec.push("loading".to_string());
        }
        if self.ammunition(key) {
            properties_vec.push(format!(
                "ammunition({}/{})",
                self.range_normal(key),
                self.range_max(key)
            ));
        }
        if self.reach(key) {
            properties_vec.push("reach".to_string());
        }
        if self.special(key) {
            properties_vec.push("special".to_string());
        }
        if self.thrown(key) {
            properties_vec.push(format!(
                "thrown({}/{})",
                self.range_normal(key),
                self.range_max(key)
            ));
        }
        if self.two_handed(key) {
            properties_vec.push("two-handed".to_string());
        }
        if self.versatile(key) {
            properties_vec.push(format!("versatile({})", self.versatile_dmg(key)));
        }

        properties_vec.sort();

        properties_str = textwrap::fill(
            &format!(
                "{} {}\n",
                properties_str,
                properties_vec.join(", ").to_case(Case::Title)
            ),
            textwrap::Options::new(COLUMN_WIDTH)
                .initial_indent("")
                .subsequent_indent("    "),
        );

        properties_str
    }

    pub fn details(&self, key: &str) -> String {
        [
            self.detail_weapon(key),
            self.detail_proficiency(key),
            self.detail_class(key),
            self.detail_cost(key),
            self.detail_damage(key),
            self.detail_weight(key),
            self.detail_properties(key),
        ]
        .join("")
    }
}
