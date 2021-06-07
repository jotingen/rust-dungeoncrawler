use crate::actor::player::classes::Classes;
use crate::actor::player::races::Races;
use crate::actor::player::Character;
use crate::actor::{Alignment, Gender};
use crate::items::weapons::Weapons;
use crate::screen::Screen;
use crate::screen::COLUMN_WIDTH;
use crate::utils::*;
use convert_case::{Case, Casing};
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
use crate::actor::player::generation::CharacterCreationState::{Variant::*, *};

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

pub fn generate(
    screen: &mut Screen,
    races: &Races,
    classes: &Classes,
    weapons: &Weapons,
) -> Character {
    let mut character: Character = Character {
        ..Default::default()
    };
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
                    screen.draw_pick_a_number("Choose number, leave blank for random.", 1, 2) - 1;

                screen.set_msg(&format!(
                    "{}\n\n{}",
                    msg,
                    if number == 0 { "M" } else { "F" }
                ));

                if screen.draw_pick_yes_or_no("Use this gender?") {
                    character.gender = if number == 0 { Gender::M } else { Gender::F };
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
                    character.race = races.keys()[number as usize].to_string();
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
                    character.class = classes.keys()[number as usize].to_string();
                    m.transition(ChooseStats).as_enum()
                } else {
                    m.transition(ChooseClass).as_enum()
                }
            }
            StatsByChooseStats(m) => {
                screen.set_header("Character Creation - Abilities");

                let mut msg: String;

                //Add in race bonuses for abilities
                character.abilities.strength = races
                    .ability_score_increase(&character.race)
                    .abilities
                    .strength;
                character.abilities.dexterity = races
                    .ability_score_increase(&character.race)
                    .abilities
                    .dexterity;
                character.abilities.charisma = races
                    .ability_score_increase(&character.race)
                    .abilities
                    .charisma;
                character.abilities.constitution = races
                    .ability_score_increase(&character.race)
                    .abilities
                    .constitution;
                character.abilities.intellect = races
                    .ability_score_increase(&character.race)
                    .abilities
                    .intellect;
                character.abilities.wisdom = races
                    .ability_score_increase(&character.race)
                    .abilities
                    .wisdom;

                msg = format!(
                    "Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n",
                    &character.abilities.strength,
                    &character.abilities.dexterity,
                    &character.abilities.charisma,
                    &character.abilities.constitution,
                    &character.abilities.intellect,
                    &character.abilities.wisdom
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
                        &character.abilities.strength,
                        &character.abilities.dexterity,
                        &character.abilities.charisma,
                        &character.abilities.constitution,
                        &character.abilities.intellect,
                        &character.abilities.wisdom
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
                        &character.abilities.strength,
                        &character.abilities.dexterity,
                        &character.abilities.charisma,
                        &character.abilities.constitution,
                        &character.abilities.intellect,
                        &character.abilities.wisdom
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
                        character.abilities.strength += stat;
                    }
                    if ability == "dexterity" {
                        character.abilities.dexterity += stat;
                    }
                    if ability == "charisma" {
                        character.abilities.charisma += stat;
                    }
                    if ability == "constitution" {
                        character.abilities.constitution += stat;
                    }
                    if ability == "intellect" {
                        character.abilities.intellect += stat;
                    }
                    if ability == "wisdom" {
                        character.abilities.wisdom += stat;
                    }
                }

                msg = format!(
                    "Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n",
                    &character.abilities.strength,
                    &character.abilities.dexterity,
                    &character.abilities.charisma,
                    &character.abilities.constitution,
                    &character.abilities.intellect,
                    &character.abilities.wisdom
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
                        if character.is_weapon_proficient(races, classes, weapons, weapon_key) {
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
                    textwrap::wrap_columns(&weapon_list, 3, COLUMN_WIDTH, "", "", "").join("\n")
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
                    character.weapons.push(
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

                screen
                    .set_msg("Type in a desired name, or leave blank for a randomly generated one");
                name = screen.draw_enter_string("Enter name:\n");
                name = name.trim().to_string();

                if name.is_empty() {
                    name = races.generate_name(&character.race, character.gender.clone());
                }

                screen.set_msg(&name);

                if screen.draw_pick_yes_or_no("Use this name?") {
                    character.name = name;
                    m.transition(ChooseSummary).as_enum()
                } else {
                    m.transition(ChooseName).as_enum()
                }
            }
            SummaryByChooseSummary(m) => {
                let mut msg: String = "".to_string();

                character.age = 25;
                character.alignment = Alignment::N;

                screen.set_header("Character Creation - Summary");

                msg = format!("{}Name: {}\n\n", msg, &character.name);

                msg = format!("{}Gender: {:?}\n\n", msg, &character.gender);

                msg = format!(
                    "{}Race: {}\n\n",
                    msg,
                    &races.detail_race(&character.race).trim()
                );

                msg = format!(
                    "{}Class: {}\n\n",
                    msg,
                    &classes.detail_class(&character.class).trim()
                );

                msg = format!("{}Alignment: {:?}\n\n", msg, &character.alignment);

                msg = format!(
                    "{}Str:{:>2} Dex:{:>2} Cha:{:>2} Con:{:>2} Int:{:>2} Wis:{:>2}\n\n",
                    msg,
                    &character.abilities.strength,
                    &character.abilities.dexterity,
                    &character.abilities.charisma,
                    &character.abilities.constitution,
                    &character.abilities.intellect,
                    &character.abilities.wisdom
                );

                let mut weapons_str: Vec<String> = Vec::new();
                for weapon in character.weapons.iter() {
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
    character
}
