use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum Tile {
    Floor,
    Wall,
}
impl Default for Tile {
    fn default() -> Self {
        Tile::Wall
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Level {
    x: usize,
    y: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Level {
    fn generate(level_number: usize) -> Level {
        //Generate level dimensions based on level
        //Start small, grow quickly, then stablize
        //y = a / (1 + b e-kx ), k > 0
        let a: f32 = 1000.0;
        let b: f32 = 20.0;
        let k: f32 = 0.75;
        let one: f32 = 1.0;
        let e: f32 = one.exp();
        let dimension_y = (a / (1.0 + b * e.powf(-k * level_number as f32))).round() as usize;
        let dimension_x = (dimension_y as f32 * 2.0).round() as usize;

        //Generate grid
        let mut level: Level = Level {
            x: dimension_x,
            y: dimension_y,
            tiles: vec![vec![Tile::Wall; dimension_x]; dimension_y],
        };

        let mut rng = rand::thread_rng();
        let minimum_room_size = 7;
        let maximum_room_size = 10;

        fn check_for_room_space(
            level: &mut Level,
            room_pos_x: usize,
            room_pos_y: usize,
            room_dimension_x: usize,
            room_dimension_y: usize,
        ) -> bool {
            //Check for edge of world
            if room_pos_x < 1
                || room_pos_y < 1
                || room_pos_x + room_dimension_x >= level.x - 1
                || room_pos_y + room_dimension_y >= level.y - 1
            {
                return false;
            }

            //Check for any empty spaces within and around room
            for x in room_pos_x - 1..=room_pos_x + room_dimension_x + 1 {
                for y in room_pos_y - 1..=room_pos_y + room_dimension_y + 1 {
                    if level.tiles[y][x] != Tile::Wall {
                        return false;
                    }
                }
            }
            true
        }

        fn carve_out_room(
            level: &mut Level,
            pos_x: usize,
            pos_y: usize,
            dimension_x: usize,
            dimension_y: usize,
        ) {
            for (y, tile_row) in level.tiles.iter_mut().enumerate() {
                for (x, tile) in tile_row.iter_mut().enumerate() {
                    if pos_x <= x
                        && pos_x + dimension_x > x
                        && pos_y <= y
                        && pos_y + dimension_y > y
                    {
                        *tile = Tile::Floor;
                    }
                }
            }
        }

        fn carve_out_doorway_and_new_room(
            level: &mut Level,
            pos_x: usize,
            pos_y: usize,
            room_dimension_x: usize,
            room_dimension_y: usize,
        ) -> bool {
            let mut rng = rand::thread_rng();

            //Check for a valid doorway is somewhere on a wall of a room, with no room on the other side

            //Never allow a doorway within 2 tiles of the edge
            if pos_x < 2 || pos_y < 2 || pos_x >= level.x - 2 || pos_y >= level.y - 2 {
                return false;
            }

            //Doorway is on stone
            if level.tiles[pos_y][pos_x] != Tile::Wall {
                return false;
            }

            //Doorway on top
            if level.tiles[pos_y + 1][pos_x] != Tile::Wall
                && level.tiles[pos_y - 1][pos_x] == Tile::Wall
            {
                //Check if room can fit on top

                //Pick random X offset and see if it can fit
                let pos_x_offset = rng.gen_range(0..room_dimension_x);
                if pos_x <= pos_x_offset || pos_x + pos_x_offset >= level.x {
                    return false;
                }
                let room_pos_x: usize = pos_x - pos_x_offset;

                if pos_y < room_dimension_y {
                    return false;
                }
                let room_pos_y: usize = pos_y - room_dimension_y;

                if check_for_room_space(
                    level,
                    room_pos_x,
                    room_pos_y,
                    room_dimension_x,
                    room_dimension_y,
                ) {
                    carve_out_room(level, pos_x, pos_y, 1, 1);
                    carve_out_room(
                        level,
                        room_pos_x,
                        room_pos_y,
                        room_dimension_x,
                        room_dimension_y,
                    );
                    return true;
                }
                return false;
            }

            //Doorway on bottom
            if level.tiles[pos_y - 1][pos_x] != Tile::Wall
                && level.tiles[pos_y + 1][pos_x] == Tile::Wall
            {
                //Check if room can fit on bottom

                //Pick random X offset and see if it can fit
                let pos_x_offset = rng.gen_range(0..room_dimension_x);
                if pos_x <= pos_x_offset || pos_x + pos_x_offset >= level.x {
                    return false;
                }
                let room_pos_x: usize = pos_x - pos_x_offset;

                if level.y <= room_dimension_y + pos_y {
                    return false;
                }
                let room_pos_y: usize = pos_y + 1;

                if check_for_room_space(
                    level,
                    room_pos_x,
                    room_pos_y,
                    room_dimension_x,
                    room_dimension_y,
                ) {
                    carve_out_room(level, pos_x, pos_y, 1, 1);
                    carve_out_room(
                        level,
                        room_pos_x,
                        room_pos_y,
                        room_dimension_x,
                        room_dimension_y,
                    );
                    return true;
                }
                return false;
            }

            //Doorway on left
            if level.tiles[pos_y][pos_x + 1] != Tile::Wall
                && level.tiles[pos_y][pos_x - 1] == Tile::Wall
            {
                //Check if room can fit on left

                if pos_x < room_dimension_x {
                    return false;
                }
                let room_pos_x: usize = pos_x - room_dimension_x;

                //Pick random Y offset and see if it can fit
                let pos_y_offset = rng.gen_range(0..room_dimension_y);
                if pos_y <= pos_y_offset || pos_y + pos_y_offset >= level.x {
                    return false;
                }
                let room_pos_y: usize = pos_y - pos_y_offset;

                if check_for_room_space(
                    level,
                    room_pos_x,
                    room_pos_y,
                    room_dimension_x,
                    room_dimension_y,
                ) {
                    carve_out_room(level, pos_x, pos_y, 1, 1);
                    carve_out_room(
                        level,
                        room_pos_x,
                        room_pos_y,
                        room_dimension_x,
                        room_dimension_y,
                    );
                    return true;
                }
                return false;
            }

            //Doorway on right
            if level.tiles[pos_y][pos_x - 1] != Tile::Wall
                && level.tiles[pos_y][pos_x + 1] == Tile::Wall
                && level.tiles[pos_y][pos_x + 2] == Tile::Wall
            {
                //Check if room can fit on right

                if level.x <= room_dimension_x + pos_x {
                    return false;
                }
                let room_pos_x: usize = pos_x + 1;

                //Pick random Y offset and see if it can fit
                let pos_y_offset = rng.gen_range(0..room_dimension_y);
                if pos_y <= pos_y_offset || pos_y + pos_y_offset >= level.x {
                    return false;
                }
                let room_pos_y: usize = pos_y - pos_y_offset;

                if check_for_room_space(
                    level,
                    room_pos_x,
                    room_pos_y,
                    room_dimension_x,
                    room_dimension_y,
                ) {
                    carve_out_room(level, pos_x, pos_y, 1, 1);
                    carve_out_room(
                        level,
                        room_pos_x,
                        room_pos_y,
                        room_dimension_x,
                        room_dimension_y,
                    );
                    return true;
                }
                return false;
            }

            false
        }

        //Build first room
        let room_dimension_x: usize = rng.gen_range(minimum_room_size..=2 * maximum_room_size);
        let room_dimension_y: usize = rng.gen_range(minimum_room_size..=maximum_room_size);
        let room_position_x: usize =
            ((dimension_x as f32) / 2.0 - (room_dimension_x as f32) / 2.0).round() as usize;
        let room_position_y: usize =
            ((dimension_y as f32) / 2.0 - (room_dimension_y as f32) / 2.0).round() as usize;
        carve_out_room(
            &mut level,
            room_position_x,
            room_position_y,
            room_dimension_x,
            room_dimension_y,
        );

        //Build next rooms
        // Give it 100k tries to find a block next to an existing room
        let mut room_count = 1;
        let mut room_created: bool;
        let mut count = 0;
        loop {
            room_created = false;
            loop {
                //Bias towards middle
                let doorway_pos_x: usize =
                    (rng.gen_range(2..=dimension_x - 2) + rng.gen_range(2..=dimension_x - 2)) / 2;
                let doorway_pos_y: usize =
                    (rng.gen_range(2..=dimension_y - 2) + rng.gen_range(2..=dimension_y - 2)) / 2;
                let room_dimension_x: usize =
                    rng.gen_range(minimum_room_size..=2 * maximum_room_size) as usize;
                let room_dimension_y: usize =
                    rng.gen_range(minimum_room_size..=maximum_room_size) as usize;

                if carve_out_doorway_and_new_room(
                    &mut level,
                    doorway_pos_x,
                    doorway_pos_y,
                    room_dimension_x,
                    room_dimension_y,
                ) {
                    count = 0;
                    room_count += 1;
                    room_created = true;
                    break;
                }

                if count > 100000 {
                    break;
                } else {
                    count += 1;
                }
            }
            if room_count > 10 + 10 * level_number || !room_created {
                break;
            }
        }

        level
    }

    pub fn can_move_to(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] != Tile::Wall
    }

    pub fn width(&self) -> usize {
        self.x
    }

    pub fn height(&self) -> usize {
        self.y
    }

    pub fn map_vec(&self) -> Vec<Vec<char>> {
            let mut map_vec = vec![vec![' '; self.x]; self.y];
            #[allow(clippy::needless_range_loop)]
            for y in 0..self.y {
                for x in 0..self.x {
                    if self.tiles[y][x] == Tile::Floor {
                        map_vec[y][x] = '.';
                    }
                    if self.tiles[y][x] == Tile::Wall {
                        map_vec[y][x] = '#';
                    }
                }
            }
            map_vec
    }

}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Levels {
    level: Vec<Level>,
}

impl Levels {
    pub fn level(&mut self, level_number: usize) -> &Level {
        if level_number >= self.level.len() {
            for number in self.level.len()..level_number + 1 {
                let new_level = Level::generate(number);
                self.level.push(new_level)
            }
        }
        &self.level[level_number]
    }
    pub fn level_start_position(&mut self, level_number:usize) -> (usize, usize) {
        (self.level(level_number).width()/2, self.level(level_number).height()/2)
    }
}
