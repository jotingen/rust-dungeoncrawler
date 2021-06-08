mod generation;

use crate::levels::level::generation::*;
use crate::utils::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum TileType {
    Floor,
    Wall,
    StairDown,
    StairUp,
}
impl Default for TileType {
    fn default() -> Self {
        TileType::Wall
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Tile {
    tile: TileType,
    seen: bool,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Level {
    columns: usize,
    rows: usize,
    tiles: Vec<Vec<Tile>>,
    exit: Point,
    entrance: Point,
}

impl Level {
    pub fn new(level_number: usize) -> Level {
        generate(level_number)
    }

    pub fn can_move_to(
        &self,
        to: Point,
        from: Point,
    ) -> bool {
        //Moving nowhere
        if to == from {
            return true;
        //Moving  1 along row or col
        } else if (to.col == from.col && (to.row as i32 - from.row as i32).abs() == 1)
            || ((to.col as i32 - from.col as i32).abs() == 1 && to.row == from.row)
        {
            return self.tiles[to.row][to.col].tile != TileType::Wall;
        //Moving diagonally
        // one of the corner squares between the points must not be a wall
        } else if (to.col as i32 - from.col as i32).abs() == 1
            && (to.row as i32 - from.row as i32).abs() == 1
        {
            return (self.tiles[from.row][to.col].tile != TileType::Wall
                || self.tiles[to.row][from.col].tile != TileType::Wall)
                && self.tiles[to.row][to.col].tile != TileType::Wall;
        }
        false
    }

    pub fn width(&self) -> usize {
        self.columns
    }

    pub fn height(&self) -> usize {
        self.rows
    }

    ///Get the initial starting position of the level
    pub fn entrance(&self) -> Point {
        self.entrance
    }

    ///Get the exit position of the level
    pub fn exit(&self) -> Point {
        self.exit
    }
    pub fn is_stair_down_at(
        &self,
        x: usize,
        y: usize,
    ) -> bool {
        self.tiles[y][x].tile == TileType::StairDown
    }

    pub fn is_stair_up_at(
        &self,
        x: usize,
        y: usize,
    ) -> bool {
        self.tiles[y][x].tile == TileType::StairUp
    }

    ///Generate map vector with symbols
    ///Updates seen vector within here
    pub fn map_vec(
        &mut self,
        player_pos_p: &Point,
    ) -> Vec<Vec<char>> {
        let mut map_vec = vec![vec![' '; self.width()]; self.height()];

        //Determine what we can see
        let mut map_visible = vec![vec![false; self.width()]; self.height()];
        //TODO Determine view distance
        //Thinking that for vision, treat each cell as 3 ft
        //darkvision can view out to 60ft
        //normal vision out to 20 ft
        //with torch out to 100ft
        let view_distance = 100 / 3;

        #[allow(clippy::needless_range_loop)]
        //Start check within a square box around the player
        for row in if player_pos_p.row >= view_distance {
            player_pos_p.row - view_distance
        } else {
            0
        }..=(player_pos_p.row + view_distance)
        {
            for col in if player_pos_p.col >= view_distance {
                player_pos_p.col - view_distance
            } else {
                0
            }..=(player_pos_p.col + view_distance)
            {
                //If cell is out of range skip to the next one
                //or if cell already visible skip to next one
                if col >= self.width() || row >= self.height() || map_visible[row][col] {
                    continue;
                }

                //Determine if distance to cell is within view range radius
                let distance: usize = (0.5 * (col as i32 - player_pos_p.col as i32).pow(2) as f32
                    + (row as i32 - player_pos_p.row as i32).pow(2) as f32)
                    .sqrt()
                    .round() as usize;
                if distance <= view_distance {
                    //Walk through vector of points from player out to point
                    for p in vec_between_points(player_pos_p, &Point { col, row }) {
                        //Mark current point as both visible and seen
                        map_visible[p.row as usize][p.col as usize] = true;
                        self.tiles[p.row as usize][p.col as usize].seen = true;

                        //If we are at a wall, we can see no further
                        if self.tiles[p.row as usize][p.col as usize].tile == TileType::Wall {
                            break;
                        }
                    }
                }
            }
        }

        #[allow(clippy::needless_range_loop)]
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.tiles[y][x].tile == TileType::Floor {
                    if map_visible[y][x] {
                        map_vec[y][x] = '.';
                    } else if self.tiles[y][x].seen {
                        map_vec[y][x] = ':';
                    }
                }
                if self.tiles[y][x].tile == TileType::StairUp
                    && (map_visible[y][x] || self.tiles[y][x].seen)
                {
                    map_vec[y][x] = '<';
                }
                if self.tiles[y][x].tile == TileType::StairDown
                    && (map_visible[y][x] || self.tiles[y][x].seen)
                {
                    map_vec[y][x] = '>';
                }
                if self.tiles[y][x].tile == TileType::Wall
                    && (map_visible[y][x] || self.tiles[y][x].seen)
                {
                    map_vec[y][x] = '#';
                }
            }
        }
        map_vec
    }
}
