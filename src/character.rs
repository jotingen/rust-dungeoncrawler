use crate::basics::{Abilities, Alignment, Gender};
use crate::classes::Classes;
use crate::races::Races;
use crate::screen::Screen;
use crate::screen::COLUMN_WIDTH;
use crate::utils::*;
use crate::weapons::{Weapon, Weapons};
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use sm::sm;

sm! {
    CharacterCreationState {
        InitialStates { Idle }

        ChooseGender {
            Idle, Summary, Gender => Gender
        }

        ChooseRace {
            Gender, Race => Race
        }

        ChooseClass {
            Race, Class => Class
        }

        ChooseStats {
            Class => Stats
        }

        ChooseEquipment {
            Stats, Equipment => Equipment
        }

        ChooseName {
            Equipment, Name => Name
        }

        ChooseSummary {
            Name => Summary
        }

        Done {
            Summary => Finished
        }
    }
}
use crate::character::CharacterCreationState::{Variant::*, *};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Character {
    pub name: String,
    pub gender: Gender,
    pub race: String,
    pub age: u32,
    pub class: String,
    pub alignment: Alignment,
    pub abilities: Abilities,
    pub weapons: Vec<Weapon>,
}

fn roll_stats(rolls: &mut Vec<u32>) {
    //println!("Rolling own");
    for roll in rolls.iter_mut() {
        let mut die_rolls: [u32; 4] = [d(6), d(6), d(6), d(6)];
        die_rolls.sort_unstable();
        die_rolls.reverse();
        *roll = die_rolls[0] + die_rolls[1] + die_rolls[2];
        //println!("Rolled {:?} for {}", die_rolls, *roll);
    }
    rolls.sort_unstable();
    rolls.reverse();
}

impl Character {
    pub fn character_creation(
        &mut self,
        screen: &mut Screen,
        races: &Races,
        classes: &Classes,
        weapons: &Weapons,
    ) {
        let mut sm = Machine::new(Idle).as_enum();
        let original_header = screen.get_header();
        loop {
            sm = match sm {
                InitialIdle(m) => {
                    screen.set_header("Character Creation");

                    screen.set_msg("Starting character creation...");
                    screen.draw_display();

                    m.transition(ChooseGender).as_enum()
                }
                GenderByChooseGender(m) => {
                    screen.set_header("Character Creation - Gender");
                    let msg = "\n\
                                     1) Male\n\
                                     2) Female";
                    screen.set_msg(msg);

                    let number =
                        screen.draw_pick_a_number("Choose number, leave blank for random.", 1, 2)
                            - 1;

                    screen.set_msg(&format!(
                        "{}\n\n{}",
                        msg,
                        if number == 0 { "M" } else { "F" }
                    ));

                    if screen.draw_pick_yes_or_no("Use this gender?") {
                        self.gender = if number == 0 { Gender::M } else { Gender::F };
                        m.transition(ChooseRace).as_enum()
                    } else {
                        m.transition(ChooseGender).as_enum()
                    }
                }
                RaceByChooseRace(m) => {
                    screen.set_header("Character Creation - Race");

                    let mut msg: String = "".to_string();
                    for (count, race_key) in races.keys().iter().enumerate() {
                        msg = format!(
                            "{}\n{:>2}) {}",
                            msg,
                            count + 1,
                            &races.detail_race(&race_key).trim()
                        );
                    }
                    screen.set_msg(&msg.strip_prefix('\n').unwrap());

                    let number = screen.draw_pick_a_number(
                        "Choose race, leave blank for random.",
                        1,
                        races.keys().len() as u32,
                    ) - 1;

                    screen.set_msg(&races.details(&races.keys()[number as usize]));

                    if screen.draw_pick_yes_or_no("Use this race?") {
                        self.race = races.keys()[number as usize].to_string();
                        m.transition(ChooseClass).as_enum()
                    } else {
                        m.transition(ChooseRace).as_enum()
                    }
                }
                ClassByChooseClass(m) => {
                    screen.set_header("Character Creation - Class");

                    let mut msg: String = "".to_string();
                    for (count, class_key) in classes.keys().iter().enumerate() {
                        msg = format!(
                            "{}\n{:>2}) {}",
                            msg,
                            count + 1,
                            &classes.detail_class(&class_key).trim()
                        );
                    }
                    screen.set_msg(&msg.strip_prefix('\n').unwrap());

                    let number = screen.draw_pick_a_number(
                        "Choose class, leave blank for random.",
                        1,
                        classes.keys().len() as u32,
                    ) - 1;

                    screen.set_msg(&classes.details(&classes.keys()[number as usize]));

                    if screen.draw_pick_yes_or_no("Use this class?") {
                        self.class = classes.keys()[number as usize].to_string();
                        m.transition(ChooseStats).as_enum()
                    } else {
                        m.transition(ChooseClass).as_enum()
                    }
                }
                StatsByChooseStats(m) => {
                    screen.set_header("Character Creation - Abilities");

                    let mut msg: String;

                    //Add in race bonuses for abilities
                    self.abilities.strength =
                        races.ability_score_increase(&self.race).abilities.strength;
                    self.abilities.dexterity =
                        races.ability_score_increase(&self.race).abilities.dexterity;
                    self.abilities.charisma =
                        races.ability_score_increase(&self.race).abilities.charisma;
                    self.abilities.constitution = races
                        .ability_score_increase(&self.race)
                        .abilities
                        .constitution;
                    self.abilities.intellect =
                        races.ability_score_increase(&self.race).abilities.intellect;
                    self.abilities.wisdom =
                        races.ability_score_increase(&self.race).abilities.wisdom;

                    msg = format!(
                        "Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n",
                        &self.abilities.strength,
                        &self.abilities.dexterity,
                        &self.abilities.charisma,
                        &self.abilities.constitution,
                        &self.abilities.intellect,
                        &self.abilities.wisdom
                    );

                    println!("Roll stats:\n");
                    msg = format!("{}\n\nYou can choose to use the default stat values or to roll random new ones.",msg);

                    let mut stats: Vec<u32> = vec![15, 14, 13, 12, 10, 8];
                    msg = format!("{}\n\nThe default stats are {:?}", msg, stats);

                    screen.set_msg(&msg);

                    if screen.draw_pick_yes_or_no("Roll your own stats?") {
                        roll_stats(&mut stats);
                    }

                    screen.set_msg(&format!("{}\n\nUsing stats {:?}", msg, stats));
                    screen.draw_display();

                    let mut abilities: Vec<&str> = vec![
                        "strength",
                        "dexterity",
                        "charisma",
                        "constitution",
                        "intellect",
                        "wisdom",
                    ];

                    while !abilities.is_empty() {
                        msg = format!(
                            "Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n",
                            &self.abilities.strength,
                            &self.abilities.dexterity,
                            &self.abilities.charisma,
                            &self.abilities.constitution,
                            &self.abilities.intellect,
                            &self.abilities.wisdom
                        );

                        msg = format!("{}\n\nChoose ability to assign stat to:", msg);

                        for (count, ability) in abilities.iter().enumerate() {
                            msg = format!(
                                "{}\n{:>2}) {}",
                                msg,
                                count + 1,
                                ability.to_case(Case::Title)
                            );
                        }

                        screen.set_msg(&msg);

                        let number = screen.draw_pick_a_number(
                            "Choose ability, leave blank for random.",
                            1,
                            abilities.len() as u32,
                        ) - 1;

                        let ability = abilities[number as usize];

                        abilities.remove(number as usize);

                        msg = format!(
                            "Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n",
                            &self.abilities.strength,
                            &self.abilities.dexterity,
                            &self.abilities.charisma,
                            &self.abilities.constitution,
                            &self.abilities.intellect,
                            &self.abilities.wisdom
                        );
                        msg = format!(
                            "{}\n\n{}\n\nChoose stat value {:?}",
                            msg,
                            ability.to_case(Case::Title),
                            stats
                        );
                        for (count, stat) in stats.iter().enumerate() {
                            msg = format!("{}\n{:>2}) {}", msg, count + 1, stat);
                        }
                        screen.set_msg(&msg);

                        let number = screen.draw_pick_a_number(
                            "Choose stat, leave blank for random.",
                            1,
                            stats.len() as u32,
                        ) - 1;

                        let stat = stats[number as usize];

                        stats.remove(number as usize);

                        if ability == "strength" {
                            self.abilities.strength += stat;
                        }
                        if ability == "dexterity" {
                            self.abilities.dexterity += stat;
                        }
                        if ability == "charisma" {
                            self.abilities.charisma += stat;
                        }
                        if ability == "constitution" {
                            self.abilities.constitution += stat;
                        }
                        if ability == "intellect" {
                            self.abilities.intellect += stat;
                        }
                        if ability == "wisdom" {
                            self.abilities.wisdom += stat;
                        }
                    }

                    msg = format!(
                        "Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n",
                        &self.abilities.strength,
                        &self.abilities.dexterity,
                        &self.abilities.charisma,
                        &self.abilities.constitution,
                        &self.abilities.intellect,
                        &self.abilities.wisdom
                    );
                    screen.set_msg(&msg);
                    screen.draw_display();

                    m.transition(ChooseEquipment).as_enum()
                }
                EquipmentByChooseEquipment(m) => {
                    let mut msg: String;

                    screen.set_header("Character Creation - Equipment - Weapon");

                    msg = "Choose equipment:".to_string();

                    let mut weapon_list: String = "".to_string();
                    for (count, weapon_key) in weapons.keys().iter().enumerate() {
                        weapon_list = format!(
                            "{}{:>2}) {} {}\n",
                            weapon_list,
                            count + 1,
                            if self.is_weapon_proficient(races, classes, weapons, weapon_key) {
                                "*"
                            } else {
                                " "
                            },
                            weapons.value(&weapon_key).unwrap().detail_name().trim()
                        );
                    }
                    msg = format!(
                        "{}\n{}",
                        msg,
                        textwrap::wrap_columns(&weapon_list, 3, COLUMN_WIDTH, "", "", "")
                            .join("\n")
                    );

                    screen.set_msg(&msg);

                    let number = screen.draw_pick_a_number(
                        "Choose weapon, leave blank for random.",
                        1,
                        weapons.keys().len() as u32,
                    ) - 1;

                    screen.set_msg(
                        &weapons
                            .value(&weapons.keys()[number as usize])
                            .unwrap()
                            .details(),
                    );

                    if screen.draw_pick_yes_or_no("Use this weapon?") {
                        self.weapons.push(
                            weapons
                                .weapon(&weapons.keys()[number as usize].to_string())
                                .unwrap(),
                        );
                        m.transition(ChooseName).as_enum()
                    } else {
                        m.transition(ChooseEquipment).as_enum()
                    }
                }
                NameByChooseName(m) => {
                    let mut name: String;

                    screen.set_header("Character Creation - Name");

                    screen.set_msg(
                        "Type in a desired name, or leave blank for a randomly generated one",
                    );
                    name = screen.draw_enter_string("Enter name:\n");
                    name = name.trim().to_string();

                    if name.is_empty() {
                        name = races.generate_name(&self.race, self.gender.clone());
                    }

                    screen.set_msg(&name);

                    if screen.draw_pick_yes_or_no("Use this name?") {
                        self.name = name;
                        m.transition(ChooseSummary).as_enum()
                    } else {
                        m.transition(ChooseName).as_enum()
                    }
                }
                SummaryByChooseSummary(m) => {
                    let mut msg: String = "".to_string();

                    self.age = 25;
                    self.alignment = Alignment::N;

                    screen.set_header("Character Creation - Summary");

                    msg = format!("{}Name: {}\n\n", msg, &self.name);

                    msg = format!("{}Gender: {:?}\n\n", msg, &self.gender);

                    msg = format!("{}Race: {}\n\n", msg, &races.detail_race(&self.race).trim());

                    msg = format!(
                        "{}Class: {}\n\n",
                        msg,
                        &classes.detail_class(&self.class).trim()
                    );

                    msg = format!("{}Alignment: {:?}\n\n", msg, &self.alignment);

                    msg = format!(
                        "{}Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n\n",
                        msg,
                        &self.abilities.strength,
                        &self.abilities.dexterity,
                        &self.abilities.charisma,
                        &self.abilities.constitution,
                        &self.abilities.intellect,
                        &self.abilities.wisdom
                    );

                    let mut weapons_str: Vec<String> = Vec::new();
                    for weapon in self.weapons.iter() {
                        weapons_str.push(weapon.detail_name());
                    }
                    msg = format!("{}Weapons: {}\n\n", msg, weapons_str.join(", "));

                    msg = format!("{}Armor: {}\n\n", msg, "-");

                    screen.set_msg(&msg);

                    if screen.draw_pick_yes_or_no("Use this character?") {
                        m.transition(Done).as_enum()
                    } else {
                        m.transition(ChooseGender).as_enum()
                    }
                }
                FinishedByDone(_) => {
                    screen.set_header(&original_header);
                    break;
                }
            }
        }
    }

    pub fn is_weapon_proficient(
        &self,
        races: &Races,
        classes: &Classes,
        weapons: &Weapons,
        weapon_key: &str,
    ) -> bool {
        //Run through race modifiers to check for weapon proficiencies
        for modifier in races.modifiers(&self.race).iter() {
            if modifier.modifier == "dwarven combat training" {
                //You have proficiency with the battleaxe, handaxe, throwing hammer, and warhammer
                if weapons.value(weapon_key).unwrap().name() == "battleaxe"
                    || weapons.value(weapon_key).unwrap().name() == "handaxe"
                    || weapons.value(weapon_key).unwrap().name() == "throwing hammer"
                    || weapons.value(weapon_key).unwrap().name() == "warhammer"
                {
                    return true;
                }
            }
            if modifier.modifier == "elf weapon training" {
                //You have proficiency with the longsword, shortsword, shortbow, and longbow.
                if weapons.value(weapon_key).unwrap().name() == "longsword"
                    || weapons.value(weapon_key).unwrap().name() == "shortsword"
                    || weapons.value(weapon_key).unwrap().name() == "shortbow"
                    || weapons.value(weapon_key).unwrap().name() == "longbow"
                {
                    return true;
                }
            }
            if modifier.modifier == "drow weapon training" {
                //You have proficiency with rapiers, shortswords, and hand crossbows.
                if weapons.value(weapon_key).unwrap().name() == "rapier"
                    || weapons.value(weapon_key).unwrap().name() == "shortsword"
                    || weapons.value(weapon_key).unwrap().name() == "hand crossbow"
                {
                    return true;
                }
            }
        }
        //Run through class to check for weapon proficiencies
        for weapon_proficiency in classes.weapon_proficiencies(&self.class).iter() {
            if &weapons.value(weapon_key).unwrap().name() == weapon_proficiency
                || &weapons.value(weapon_key).unwrap().proficiency() == weapon_proficiency
            {
                return true;
            }
        }

        false
    }
}
