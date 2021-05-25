const COLUMN_WIDTH: usize = 80;
const ROW_HEIGHT: usize = 25;

mod basics;
mod character;
mod classes;
mod races;
mod screen;
mod utils;
mod weapons;

use crate::character::Character;
use crate::classes::Classes;
use crate::races::Races;
use crate::screen::Screen;
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
    //Load screen
    let mut screen: Screen = Screen::new();

    //Load races
    let races: Races = Races::new();

    //Load classes
    let classes: Classes = Classes::new();

    //Load weapons
    let weapons: Weapons = Weapons::new();

    let mut character: Character = Character::new();

    let mut sm = Machine::new(Idle).as_enum();
    loop {
        sm = match sm {
            InitialIdle(m) => {
                screen.set_header("Dungeon Crawler");

                screen.set_msg("Welcome to Dungeon Crawler");
                screen.draw();
                pause();

                if true {
                    screen.set_msg("No savegame found, starting new game...");
                    screen.draw();
                    pause();

                    m.transition(CreateCharacter).as_enum()
                } else {
                    screen.set_msg("Loading game...");
                    screen.draw();
                    pause();

                    m.transition(LoadGame).as_enum()
                }
            }

            LoadByLoadGame(m) => m.transition(LaunchGame).as_enum(),

            CharacterByCreateCharacter(m) => {
                clear();
                character.character_creation(&mut screen,&races, &classes, &weapons);
                m.transition(LaunchGame).as_enum()
            }

            GameByLaunchGame(m) => {
                clear();

                pause();

                m.transition(Done).as_enum()
            }

            FinishedByDone(_) => {
                    screen.set_msg("Thanks for playing");
                    screen.draw();
                    pause();

                break;
            }
        }
    }
}
