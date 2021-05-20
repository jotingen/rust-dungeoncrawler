const COLUMN_WIDTH: usize = 80;

mod basics;
mod character;
mod classes;
mod races;
mod utils;
mod weapons;

use crate::basics::{Abilities, Alignment};
use crate::character::Character;
use crate::classes::Classes;
use crate::races::Races;
use crate::utils::*;
use crate::weapons::Weapons;
use sm::sm;

sm! {
    GameState {
        InitialStates { Idle }

        LoadGame {
            Idle => Load
        }

        CreateCharacter {
            Idle => Character
        }

        LaunchGame {
            Load, Character => Game
        }

        Done {
            Game => Finished
        }
    }
}
use crate::GameState::{Variant::*, *};

fn main() {
    //Load races
    let races: Races = Races::new();

    //Load classes
    let classes: Classes = Classes::new();

    //Load weapons
    let weapons: Weapons = Weapons::new();

    let mut character: Character;

    let mut sm = Machine::new(Idle).as_enum();
    loop {
        sm = match sm {
            InitialIdle(m) => {
                clear();

                println!("Dungeon Crawler\n\n");

                if true {
                    println!("No savegame found, starting new game..\n\n");

                    pause();

                    m.transition(CreateCharacter).as_enum()
                } else {
                    println!("Loading game..\n\n");

                    pause();

                    m.transition(LoadGame).as_enum()
                }
            }

            LoadByLoadGame(m) => m.transition(LaunchGame).as_enum(),

            CharacterByCreateCharacter(m) => {
                clear();
                character = Character::character_creation(&races, &classes);
                m.transition(LaunchGame).as_enum()
            }

            GameByLaunchGame(m) => {
                clear();

                pause();

                m.transition(Done).as_enum()
            }

            FinishedByDone(_) => {
                println! {"Done"};
                break;
            }
        }
    }
}
