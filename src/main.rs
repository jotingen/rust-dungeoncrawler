mod actor;
mod game;
mod levels;
mod screen;
mod utils;
mod items;


use crate::actor::player::classes::Classes;
use crate::game::Game;
use crate::actor::player::races::Races;
use crate::screen::Screen;
use crate::items::weapons::Weapons;
use sm::sm;
use std::env;
use std::path::Path;

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
    //Process args
    let args: Vec<String> = env::args().collect();

    //Process save file name
    let save_file: String = if args.len() == 2 {
        args[1].to_string()
    } else {
        "default_save.json".to_string()
    };

    //Load screen
    let mut screen: Screen = Screen::new();

    //Load races
    let races: Races = Races::new();

    //Load classes
    let classes: Classes = Classes::new();

    //Load weapons
    let weapons: Weapons = Weapons::new();

    let mut game: Game = Game::new();

    let mut sm = Machine::new(Idle).as_enum();
    loop {
        sm = match sm {
            InitialIdle(m) => {
                screen.set_header("Dungeon Crawler");

                screen.set_msg("Welcome to Dungeon Crawler");
                screen.draw_display();

                if Path::new(&save_file).exists() {
                    screen.set_msg(&format!("Loading game from '{}'", save_file));
                    screen.draw_display();

                    m.transition(LoadGame).as_enum()
                } else {
                    screen.set_msg(&format!(
                        "No savegame found, starting new game using '{}'",
                        save_file
                    ));
                    screen.draw_display();

                    m.transition(CreateCharacter).as_enum()
                }
            }

            LoadByLoadGame(m) => {
                game.load(&save_file);
                m.transition(LaunchGame).as_enum()
            }

            CharacterByCreateCharacter(m) => {
                game.character
                    .new(&mut screen, &races, &classes, &weapons);
                game.save(&save_file);
                m.transition(LaunchGame).as_enum()
            }

            GameByLaunchGame(m) => {
                game.run(&mut screen);

                m.transition(Done).as_enum()
            }

            FinishedByDone(_) => {
                screen.set_msg("Thanks for playing");
                screen.draw_display();

                break;
            }
        }
    }
}
