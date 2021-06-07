use crate::levels::level::*;
use crate::utils::*;
use rand::Rng;

///Generate level dimensions based on level number
fn generate_width_and_height(level_number: usize) -> (usize, usize) {
    //Start small, grow quickly, then stablize
    //y = a / (1 + b e-kx ), k > 0
    let a: f32 = 1000.0;
    let b: f32 = 10.0;
    let k: f32 = 0.75;
    let one: f32 = 1.0;
    let e: f32 = one.exp();
    let height = (a / (1.0 + b * e.powf(-k * level_number as f32))).round() as usize;
    let width = (height as f32 * 2.0).round() as usize;
    (width, height)
}

fn room_collision(
    level: &mut Level,
    room_pos_x: usize,
    room_pos_y: usize,
    room_dimension_x: usize,
    room_dimension_y: usize,
) -> bool {
    //Check for edge of world
    if room_pos_x < 1
        || room_pos_y < 1
        || room_pos_x + room_dimension_x >= level.columns - 1
        || room_pos_y + room_dimension_y >= level.rows - 1
    {
        return false;
    }

    //Check for any empty spaces within and around room
    for x in room_pos_x - 1..=room_pos_x + room_dimension_x + 1 {
        for y in room_pos_y - 1..=room_pos_y + room_dimension_y + 1 {
            if level.tiles[y][x].tile != TileType::Wall {
                return false;
            }
        }
    }
    true
}

fn empty_out_area(
    level: &mut Level,
    pos_x: usize,
    pos_y: usize,
    dimension_x: usize,
    dimension_y: usize,
) {
    for (y, tile_row) in level.tiles.iter_mut().enumerate() {
        for (x, tile) in tile_row.iter_mut().enumerate() {
            if pos_x <= x && pos_x + dimension_x > x && pos_y <= y && pos_y + dimension_y > y {
                tile.tile = TileType::Floor;
            }
        }
    }
}

fn empty_out_hallway_and_new_room(
    level: &mut Level,
    pos_x: usize,
    pos_y: usize,
    room_dimension_x: usize,
    room_dimension_y: usize,
) -> bool {
    let mut rng = rand::thread_rng();

    //Check for a valid hallway is somewhere on a wall of a room, with no room on the other side

    //Never allow a hallway within 2 tiles of the edge
    if pos_x < 2 || pos_y < 2 || pos_x >= level.columns - 2 || pos_y >= level.rows - 2 {
        return false;
    }

    //Hallway is on stone
    if level.tiles[pos_y][pos_x].tile != TileType::Wall {
        return false;
    }

    //Set hallway length
    //95% of the time it should be length 2, long enough for there to be space between rooms
    //5% randomize to something longer
    let hallway_length = if rng.gen_bool(0.95) {
        2
    } else {
        rng.gen_range(20..50)
    };

    //Hallway on top
    if level.tiles[pos_y + 1][pos_x].tile != TileType::Wall
        && level.tiles[pos_y - 1][pos_x].tile == TileType::Wall
    {
        //Check if room can fit on top

        //Pick random X offset and see if it can fit
        let pos_x_offset = rng.gen_range(0..room_dimension_x);
        if pos_x <= pos_x_offset || pos_x + pos_x_offset >= level.columns {
            return false;
        }
        let room_pos_x: usize = pos_x - pos_x_offset;

        if pos_y < hallway_length + room_dimension_y {
            return false;
        }
        let room_pos_y: usize = pos_y - hallway_length - room_dimension_y;

        if room_collision(level, pos_x, pos_y - hallway_length, 1, hallway_length - 1)
            && room_collision(
                level,
                room_pos_x,
                room_pos_y,
                room_dimension_x,
                room_dimension_y,
            )
        {
            empty_out_area(level, pos_x, pos_y - hallway_length, 1, hallway_length + 1);
            empty_out_area(
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

    //Hallway on bottom
    if level.tiles[pos_y - 1][pos_x].tile != TileType::Wall
        && level.tiles[pos_y + 1][pos_x].tile == TileType::Wall
    {
        //Check if room can fit on bottom

        //Pick random X offset and see if it can fit
        let pos_x_offset = rng.gen_range(0..room_dimension_x);
        if pos_x <= pos_x_offset || pos_x + pos_x_offset >= level.columns {
            return false;
        }
        let room_pos_x: usize = pos_x - pos_x_offset;

        if level.rows <= room_dimension_y + pos_y + hallway_length {
            return false;
        }
        let room_pos_y: usize = pos_y + hallway_length;

        if room_collision(level, pos_x, pos_y - 1, 1, hallway_length + 1)
            && room_collision(
                level,
                room_pos_x,
                room_pos_y,
                room_dimension_x,
                room_dimension_y,
            )
        {
            empty_out_area(level, pos_x, pos_y, 1, hallway_length);
            empty_out_area(
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

    //Hallway on left
    if level.tiles[pos_y][pos_x + 1].tile != TileType::Wall
        && level.tiles[pos_y][pos_x - 1].tile == TileType::Wall
    {
        //Check if room can fit on left

        if pos_x < hallway_length + room_dimension_x {
            return false;
        }
        let room_pos_x: usize = pos_x - hallway_length - room_dimension_x;

        //Pick random Y offset and see if it can fit
        let pos_y_offset = rng.gen_range(0..room_dimension_y);
        if pos_y <= pos_y_offset || pos_y + pos_y_offset >= level.columns {
            return false;
        }
        let room_pos_y: usize = pos_y - pos_y_offset;

        if room_collision(level, pos_x - hallway_length, pos_y, hallway_length - 1, 1)
            && room_collision(
                level,
                room_pos_x,
                room_pos_y,
                room_dimension_x,
                room_dimension_y,
            )
        {
            empty_out_area(level, pos_x - hallway_length, pos_y, hallway_length + 1, 1);
            empty_out_area(
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

    //Hallway on right
    if level.tiles[pos_y][pos_x - 1].tile != TileType::Wall
        && level.tiles[pos_y][pos_x + 1].tile == TileType::Wall
        && level.tiles[pos_y][pos_x + 2].tile == TileType::Wall
    {
        //Check if room can fit on right

        if level.columns <= room_dimension_x + pos_x + hallway_length {
            return false;
        }
        let room_pos_x: usize = pos_x + hallway_length + 1;

        //Pick random Y offset and see if it can fit
        let pos_y_offset = rng.gen_range(0..room_dimension_y);
        if pos_y <= pos_y_offset || pos_y + pos_y_offset >= level.columns {
            return false;
        }
        let room_pos_y: usize = pos_y - pos_y_offset;

        if room_collision(level, pos_x + 1, pos_y, hallway_length + 1, 1)
            && room_collision(
                level,
                room_pos_x,
                room_pos_y,
                room_dimension_x,
                room_dimension_y,
            )
        {
            empty_out_area(level, pos_x, pos_y, hallway_length + 1, 1);
            empty_out_area(
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

pub fn generate(level_number: usize) -> Level {
    let (width, height) = generate_width_and_height(level_number);

    //Create Level, full of unseen walls
    let mut level: Level = Level {
        columns: width,
        rows: height,
        tiles: vec![
            vec![
                Tile {
                    tile: TileType::Wall,
                    seen: false
                };
                width
            ];
            height
        ],
        exit: Point { col: 0, row: 0 }, //TMP
        entrance: Point {
            col: width / 2,
            row: height / 2,
        },
    };

    let mut rng = rand::thread_rng();
    let minimum_room_size = 7;
    let maximum_room_size = 10;

    //Check if room collides with either the edge of the map or a non-walled space

    //Build first room
    //Make it near the center
    let room_dimension_x: usize = rng.gen_range(minimum_room_size..=2 * maximum_room_size);
    let room_dimension_y: usize = rng.gen_range(minimum_room_size..=maximum_room_size);
    let room_position_x: usize =
        ((width as f32) / 2.0 - (room_dimension_x as f32) / 2.0).round() as usize;
    let room_position_y: usize =
        ((height as f32) / 2.0 - (room_dimension_y as f32) / 2.0).round() as usize;
    empty_out_area(
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
            let hallway_pos_x: usize = rng.gen_range(2..=width - 2);
            let hallway_pos_y: usize = rng.gen_range(2..=height - 2);
            let room_dimension_x: usize =
                rng.gen_range(minimum_room_size..=2 * maximum_room_size) as usize;
            let room_dimension_y: usize =
                rng.gen_range(minimum_room_size..=maximum_room_size) as usize;

            if empty_out_hallway_and_new_room(
                &mut level,
                hallway_pos_x,
                hallway_pos_y,
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
        if room_count > 10 + 50 * level_number || !room_created {
            break;
        }
    }

    //Build stairs up if not level 0
    if level_number != 0 {
        level.tiles[level.entrance.row as usize][level.entrance.col as usize].tile =
            TileType::StairUp;
    }

    //Build stairs down somewhere
    //Must have one, keep looping until we find space
    loop {
        let stairway_down_pos_x: usize = rng.gen_range(2..=width - 2);
        let stairway_down_pos_y: usize = rng.gen_range(2..=height - 2);

        //Want a 3x3 empty space
        if level.tiles[stairway_down_pos_y - 1][stairway_down_pos_x - 1].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y - 1][stairway_down_pos_x].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y - 1][stairway_down_pos_x + 1].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y][stairway_down_pos_x - 1].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y][stairway_down_pos_x].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y][stairway_down_pos_x + 1].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y + 1][stairway_down_pos_x - 1].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y + 1][stairway_down_pos_x].tile == TileType::Floor
            && level.tiles[stairway_down_pos_y + 1][stairway_down_pos_x + 1].tile == TileType::Floor
        {
            level.exit = Point {
                col: stairway_down_pos_x,
                row: stairway_down_pos_y,
            };
            level.tiles[stairway_down_pos_y][stairway_down_pos_x].tile = TileType::StairDown;
            break;
        }
    }

    level
}
