mod level;

use serde::{Deserialize, Serialize};

use crate::levels::level::*;
use crate::utils::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Levels {
    level: Vec<Level>,
}

impl Levels {
    pub fn level(
        &mut self,
        level_number: usize,
    ) -> &mut Level {
        if level_number >= self.level.len() {
            for number in self.level.len()..level_number + 1 {
                let new_level = Level::new(number);
                self.level.push(new_level)
            }
        }
        &mut self.level[level_number]
    }

    ///Get the initial starting position of the level
    pub fn level_start_position(
        &mut self,
        level_number: usize,
    ) -> Point {
        self.level(level_number).entrance()
    }

    ///Get the exit position of the level
    pub fn level_exit_position(
        &mut self,
        level_number: usize,
    ) -> Point {
        self.level(level_number).exit()
    }

    ///Generate map vector with symbols
    pub fn map_vec(
        &mut self,
        level_number: usize,
        player_pos_p: &Point,
    ) -> Vec<Vec<char>> {
        self.level(level_number).map_vec(player_pos_p)
    }
}
