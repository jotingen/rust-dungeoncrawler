use crate::screen::COLUMN_WIDTH;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Weapon {
    weapon: String,
    proficiency: String,
    class: String,
    cost: u32,
    damage: String,
    weight: f32,
    properties: WeaponProperties,
}

impl Weapon {
    pub fn name(&self) -> String {
        self.weapon.clone()
    }

    pub fn proficiency(&self) -> String {
        self.proficiency.clone()
    }

    pub fn class(&self) -> String {
        self.class.clone()
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }

    pub fn damage(&self) -> String {
        self.damage.clone()
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn finesse(&self) -> bool {
        self.properties.finesse
    }

    pub fn heavy(&self) -> bool {
        self.properties.heavy
    }

    pub fn light(&self) -> bool {
        self.properties.light
    }

    pub fn loading(&self) -> bool {
        self.properties.loading
    }

    pub fn ammunition(&self) -> bool {
        self.properties.ammunition
    }

    pub fn range_normal(&self) -> u32 {
        self.properties.range_normal
    }

    pub fn range_max(&self) -> u32 {
        self.properties.range_max
    }

    pub fn reach(&self) -> bool {
        self.properties.reach
    }

    pub fn special(&self) -> bool {
        self.properties.special
    }

    pub fn thrown(&self) -> bool {
        self.properties.thrown
    }

    pub fn two_handed(&self) -> bool {
        self.properties.two_handed
    }

    pub fn versatile(&self) -> bool {
        self.properties.versatile
    }

    pub fn versatile_dmg(&self) -> String {
        self.properties.versatile_dmg.clone()
    }

    pub fn detail_name(&self) -> String {
        format!("{}\n", self.name().to_case(Case::Title))
    }

    fn detail_proficiency(&self) -> String {
        let proficiency = self.proficiency();
        let mut proficiency_str: String;

        proficiency_str = "- Proficiency:".to_string();

        proficiency_str = format!("{} {}\n", proficiency_str, proficiency.to_case(Case::Title));

        proficiency_str
    }

    fn detail_class(&self) -> String {
        let class = self.class();
        let mut class_str: String;

        class_str = "- Class:".to_string();

        class_str = format!("{} {}\n", class_str, class.to_case(Case::Title));

        class_str
    }

    fn detail_cost(&self) -> String {
        let cost = self.cost();
        let mut cost_str: String;

        cost_str = "- Cost:".to_string();

        cost_str = format!("{} {} cp\n", cost_str, cost);

        cost_str
    }

    fn detail_damage(&self) -> String {
        let damage = self.damage();
        let mut damage_str: String;

        damage_str = "- Damage:".to_string();

        damage_str = format!("{} {}\n", damage_str, damage.to_case(Case::Title));

        damage_str
    }

    fn detail_weight(&self) -> String {
        let weight = self.weight();
        let mut weight_str: String;

        weight_str = "- Weight:".to_string();

        weight_str = format!("{} {} lb\n", weight_str, weight);

        weight_str
    }

    fn detail_properties(&self) -> String {
        let mut properties_str: String;

        properties_str = "- Properties:".to_string();

        let mut properties_vec: Vec<String> = Vec::new();

        if self.finesse() {
            properties_vec.push("finesse".to_string());
        }
        if self.heavy() {
            properties_vec.push("heavy".to_string());
        }
        if self.light() {
            properties_vec.push("light".to_string());
        }
        if self.loading() {
            properties_vec.push("loading".to_string());
        }
        if self.ammunition() {
            properties_vec.push(format!(
                "ammunition({}/{})",
                self.range_normal(),
                self.range_max()
            ));
        }
        if self.reach() {
            properties_vec.push("reach".to_string());
        }
        if self.special() {
            properties_vec.push("special".to_string());
        }
        if self.thrown() {
            properties_vec.push(format!(
                "thrown({}/{})",
                self.range_normal(),
                self.range_max()
            ));
        }
        if self.two_handed() {
            properties_vec.push("two-handed".to_string());
        }
        if self.versatile() {
            properties_vec.push(format!("versatile({})", self.versatile_dmg()));
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

    pub fn details(&self) -> String {
        [
            self.detail_name(),
            self.detail_proficiency(),
            self.detail_class(),
            self.detail_cost(),
            self.detail_damage(),
            self.detail_weight(),
            self.detail_properties(),
        ]
        .join("")
    }
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

    pub fn value(
        &self,
        key: &str,
    ) -> Option<&Weapon> {
        for weapon in &self.weapons {
            if weapon.weapon == key {
                return Some(weapon);
            }
        }
        None
    }

    pub fn weapon(
        &self,
        key: &str,
    ) -> Option<Weapon> {
        for weapon in self.weapons.iter() {
            if weapon.weapon == key {
                return Some(weapon.clone());
            }
        }
        None
    }
}
