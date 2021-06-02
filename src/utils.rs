use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::Rng;
use regex::Regex;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

///Struct indicating a point on the game grid
///
///Can attempt to draw below point 0, so i32 are used
pub struct Point {
    pub x: i32,
    pub y: i32,
}

///Dice roll
///
///Returns random number, size of dice is provided
pub fn d(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    rng.gen_range(1..(num + 1))
}

///Clears the console
pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}

///Prompts user to press enter to continue
pub fn pause() {
    let mut stdout = stdout();
    print!("Press Enter to continue...");
    stdout.flush().unwrap();
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).unwrap();
}

///Prompts user to pick yes or not
///
///If an empty response is given, assumes yes
pub fn pick_yes_or_no(msg: &str) -> bool {
    let mut stdout = stdout();
    print!("{} (Y/n) ", msg);
    stdout.flush().unwrap();
    let mut my_yes_or_no_str = String::new();
    stdin().read_line(&mut my_yes_or_no_str).unwrap();

    //regex for empty/y*/Y*
    let re_yes = Regex::new(r"^(?i)\s*y(es)?\s*$").unwrap();
    if re_yes.is_match(&my_yes_or_no_str) || my_yes_or_no_str.trim().is_empty() {
        return true;
    }
    false
}

///Prompts user to enter a string
pub fn enter_string(msg: &str) -> String {
    let mut stdout = stdout();
    if !msg.is_empty() {
        print!("{}", msg);
    }
    stdout.flush().unwrap();
    let mut my_str = String::new();
    stdin().read_line(&mut my_str).unwrap();

    my_str
}

///Prompts user to enter a character
#[allow(clippy::never_loop)]
pub fn enter_char(msg: &str) -> char {
    let mut stdout = stdout();
    if !msg.is_empty() {
        print!("{}", msg);
    }
    stdout.flush().unwrap();
    enable_raw_mode().unwrap();
    let mut my_char: char = ' ';

    if poll(Duration::from_millis(1_000)).unwrap() {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) = read().unwrap()
        {
            my_char = c;
            disable_raw_mode().unwrap();
            return my_char;
        }
    }
    disable_raw_mode().unwrap();
    my_char
}

///Prompts user to pick a number in a given range
///
///If an empty string is given, a random number is chosen
///If a non-number is given, the user is reprompted
pub fn pick_number(
    msg: &str,
    low: u32,
    high: u32,
) -> u32 {
    let mut stdout = stdout();
    loop {
        if !msg.is_empty() {
            print!("{} ", msg);
        }
        print!("({}-{}) ", low, high);
        stdout.flush().unwrap();
        let mut my_number_str = String::new();
        stdin().read_line(&mut my_number_str).unwrap();

        if my_number_str.trim().is_empty() {
            let mut rng = rand::thread_rng();
            return rng.gen_range(low..(high + 1));
        } else if my_number_str.trim().parse::<u32>().is_ok() {
            let my_number: u32 = my_number_str.trim().parse().unwrap();
            if my_number >= low && my_number <= high {
                return my_number;
            }
        }
    }
}

///Counts the number of newlines in a string
pub fn count_newlines(msg: &str) -> u32 {
    let mut count = 0;
    for c in msg.graphemes(true) {
        if c == "\n" {
            count += 1;
        }
    }
    count
}

/// Generates a vector of points from (x0,y0) to (x1,y1)
///
/// Taken from Wikipedia: [https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm]()
pub fn vec_between_points(
    p0: &Point,
    p1: &Point,
) -> Vec<Point> {
    if (p1.y - p0.y).abs() < (p1.x - p0.x).abs() {
        if p0.x > p1.x {
            vec_between_points_low(p1, p0).into_iter().rev().collect()
        } else {
            vec_between_points_low(p0, p1)
        }
    } else if p0.y > p1.y {
        vec_between_points_high(p1, p0).into_iter().rev().collect()
    } else {
        vec_between_points_high(p0, p1)
    }
}

fn vec_between_points_low(
    p0: &Point,
    p1: &Point,
) -> Vec<Point> {
    let dx = p1.x - p0.x;
    let mut dy = p1.y - p0.y;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut diff = (2 * dy) - dx;
    let mut y = p0.y;

    let mut line: Vec<Point> = Vec::new();
    for x in p0.x..=p1.x {
        line.push(Point { x, y });
        if diff > 0 {
            y += yi;
            diff += 2 * (dy - dx);
        } else {
            diff += 2 * dy;
        }
    }
    line
}

fn vec_between_points_high(
    p0: &Point,
    p1: &Point,
) -> Vec<Point> {
    let mut dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut diff = (2 * dx) - dy;
    let mut x = p0.x;

    let mut line: Vec<Point> = Vec::new();
    for y in p0.y..=p1.y {
        line.push(Point { x, y });
        if diff > 0 {
            x += xi;
            diff += 2 * (dx - dy);
        } else {
            diff += 2 * dx;
        }
    }
    line
}

pub fn strip_trailing_newline(input: &str) -> &str {
    return input.strip_suffix("\r\n")
           .or_else(|| input.strip_suffix("\n"))
           .unwrap_or(&input);
}