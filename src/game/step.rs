use crate::game::Game;
use crate::screen::Screen;
use crate::utils::*;

pub fn step(
    game: &mut Game,
    screen: &mut Screen,
) -> bool {
    //Increment AP points for actor and creatures
    game.character.action_points_increment(1);

    //If actor has met AP threshold, get input and process move
    //Loop until action points are used up
    while game.character.action_points() >= 2 {
        match screen.draw_enter_char("Move: wasd/ykuhbjnl Interact: <space> Nothing: . Quit: q") {
            //force refresh
            'r' => screen.force_refresh(),
            //Player chose to quit
            'q' => return true,
            //Player chose to do nothing more
            '.' => break,
            //Move forward/left
            'y' => {
                if game.position.x != 0
                    && game.position.y != 0
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: (game.position.x - 1) as usize,
                                row: (game.position.y - 1) as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.y -= 1;
                    game.position.x -= 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Move forward
            'w' | 'k' => {
                if game.position.y != 0
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: game.position.x as usize,
                                row: (game.position.y - 1) as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.y -= 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Move forward/right
            'u' => {
                if game.position.x
                    != game
                        .levels
                        .level(game.position.level_number as usize)
                        .width() as i32
                        - 1
                    && game.position.y != 0
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: (game.position.x + 1) as usize,
                                row: (game.position.y - 1) as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.y -= 1;
                    game.position.x += 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Move left
            'a' | 'h' => {
                if game.position.x != 0
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: (game.position.x - 1) as usize,
                                row: game.position.y as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.x -= 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Move right
            'd' | 'l' => {
                if game.position.x
                    != game
                        .levels
                        .level(game.position.level_number as usize)
                        .width() as i32
                        - 1
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: (game.position.x + 1) as usize,
                                row: game.position.y as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.x += 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Move down/left
            'b' => {
                if game.position.x != 0
                    && game.position.y
                        != game
                            .levels
                            .level(game.position.level_number as usize)
                            .height() as i32
                            - 1
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: (game.position.x - 1) as usize,
                                row: (game.position.y + 1) as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.y += 1;
                    game.position.x -= 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Move down
            's' | 'j' => {
                if game.position.y
                    != game
                        .levels
                        .level(game.position.level_number as usize)
                        .height() as i32
                        - 1
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: game.position.x as usize,
                                row: (game.position.y + 1) as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.y += 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Move down/right
            'n' => {
                if game.position.x
                    != game
                        .levels
                        .level(game.position.level_number as usize)
                        .width() as i32
                        - 1
                    && game.position.y
                        != game
                            .levels
                            .level(game.position.level_number as usize)
                            .height() as i32
                            - 1
                    && game
                        .levels
                        .level(game.position.level_number as usize)
                        .can_move_to(
                            Point {
                                col: (game.position.x + 1) as usize,
                                row: (game.position.y + 1) as usize,
                            },
                            Point {
                                col: (game.position.x) as usize,
                                row: game.position.y as usize,
                            },
                        )
                {
                    game.position.y += 1;
                    game.position.x += 1;
                    game.character.action_points_decrement(2);
                }
            }
            //Interact
            ' ' => {
                //Note: Use if else to avoid going down/up stairs, and for other future possible collisions

                //Stairs Down
                if game
                    .levels
                    .level(game.position.level_number as usize)
                    .is_stair_down_at(game.position.x as usize, game.position.y as usize)
                {
                    game.position.level_number += 1;
                    game.levels.level(game.position.level_number as usize); //Make sure level has been generated
                    let position_p = game
                        .levels
                        .level_start_position(game.position.level_number as usize);
                    game.position.x = position_p.col as i32;
                    game.position.y = position_p.row as i32;
                    game.character.action_points_decrement(2);
                }
                //Stairs Up
                else if game
                    .levels
                    .level(game.position.level_number as usize)
                    .is_stair_up_at(game.position.x as usize, game.position.y as usize)
                    && game.position.level_number > 0
                {
                    game.position.level_number -= 1;
                    game.levels.level(game.position.level_number as usize);
                    let position_p = game
                        .levels
                        .level_exit_position(game.position.level_number as usize);
                    game.position.x = position_p.col as i32;
                    game.position.y = position_p.row as i32;
                    game.character.action_points_decrement(2);
                }
            }
            //Default, unrecognized key
            _ => {}
        }
    }

    //If creature has met AP threshold, get input and process move

    //Increment AP points for next turn
    game.character.action_points_increment(1);

    false
}
