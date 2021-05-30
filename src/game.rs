use crate::character::Character;
use crate::levels::Levels;
use crate::screen::Screen;
use serde::{Deserialize, Serialize};
use sm::sm;
use std::fs;

sm! {
    GameState {
        InitialStates { Idle }

        ChooseNavigate {
            Idle, Navigate => Navigate
        }

        Done {
            Navigate => Finished
        }
    }
}
use crate::game::GameState::{Variant::*, *};

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    level_number: i32,
    x: u32,
    y: u32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            level_number: -1,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Game {
    pub character: Character,
    pub levels: Levels,
    position: Position,
}

impl Game {
    pub fn new() -> Game {
        Game {
            ..Default::default()
        }
    }

    pub fn save(&self, file: &str) {
        let game_str = serde_json::to_string_pretty(&self).unwrap();
        fs::write(file, game_str).expect("Unable to save file");
    }

    pub fn load(&mut self, file: &str) {
        let game_str = fs::read_to_string(&file).expect("Unable to open file");
        *self = serde_json::from_str(&game_str).unwrap();
    }

    pub fn run(&mut self, screen: &mut Screen) {
        let mut sm = Machine::new(Idle).as_enum();
        let original_header = screen.get_header();
        loop {
            sm = match sm {
                InitialIdle(m) => {
                    screen.set_header(&original_header);

                    screen.set_msg("Entering into dungeon...");
                    screen.draw_display();

                    m.transition(ChooseNavigate).as_enum()
                }
                NavigateByChooseNavigate(m) => {
                    //Generate level 0, set position
                    if self.position.level_number == -1 {
                        self.position.level_number = 0;
                        self.levels.level(self.position.level_number as usize);
                        let (position_x, position_y) = self
                            .levels
                            .level_start_position(self.position.level_number as usize);
                        self.position.x = position_x as u32;
                        self.position.y = position_y as u32;
                    }
                    screen.set_map(
                        self.levels
                            .map_vec(self.position.level_number as usize,self.position.x as usize, self.position.y as usize),
                        self.position.x,
                        self.position.y,
                    );
                    let input_char = screen.draw_enter_char("Move: w/a/s/d Quit: q");

                    if input_char == 'w'
                        && self.position.y != 0
                        && self
                            .levels
                            .level(self.position.level_number as usize)
                            .can_move_to(self.position.x as usize, (self.position.y - 1) as usize)
                    {
                        self.position.y -= 1;
                    }
                    if input_char == 'a'
                        && self.position.x != 0
                        && self
                            .levels
                            .level(self.position.level_number as usize)
                            .can_move_to((self.position.x - 1) as usize, self.position.y as usize)
                    {
                        self.position.x -= 1;
                    }
                    if input_char == 's'
                        && self.position.y
                            != self
                                .levels
                                .level(self.position.level_number as usize)
                                .height() as u32
                                - 1
                        && self
                            .levels
                            .level(self.position.level_number as usize)
                            .can_move_to(self.position.x as usize, (self.position.y + 1) as usize)
                    {
                        self.position.y += 1;
                    }
                    if input_char == 'd'
                        && self.position.x
                            != self
                                .levels
                                .level(self.position.level_number as usize)
                                .width() as u32
                                - 1
                        && self
                            .levels
                            .level(self.position.level_number as usize)
                            .can_move_to((self.position.x + 1) as usize, self.position.y as usize)
                    {
                        self.position.x += 1;
                    }

                    if input_char == 'q' {
                        m.transition(Done).as_enum()
                    } else {
                        m.transition(ChooseNavigate).as_enum()
                    }
                }
                FinishedByDone(_) => {
                    screen.set_header(&original_header);
                    break;
                }
            }
        }
    }
}
