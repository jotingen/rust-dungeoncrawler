use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{self, stdin, stdout, Read, Write};
use unicode_segmentation::UnicodeSegmentation;

///Struct indicating a point on the game grid
#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub col: usize,
    pub row: usize,
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
    stdout.flush().unwrap();
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).unwrap();
}

///Prompts user to pick yes or not
///
///If an empty response is given, assumes yes
pub fn pick_yes_or_no() -> bool {
    let mut stdout = stdout();
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
pub fn enter_string() -> String {
    let mut stdout = stdout();
    stdout.flush().unwrap();
    let mut my_str = String::new();
    stdin().read_line(&mut my_str).unwrap();

    my_str
}

///Prompts user to enter a character
#[allow(clippy::never_loop)]
pub fn enter_char() -> char {
    let mut stdout = stdout();
    stdout.flush().unwrap();
    enable_raw_mode().unwrap();
    let mut my_char: char = '~'; //Unused character

    for b in io::stdin().bytes() {
        my_char = b.unwrap() as char;
        //Clippy does not like returning from here
        //but I only want a character, without
        //this it seems to keep polling for characters
        disable_raw_mode().unwrap();
        return my_char;
    }
    disable_raw_mode().unwrap();
    my_char
}

///Prompts user to pick a number in a given range
///
///If an empty string is given, a random number is chosen
///If a non-number is given, the user is reprompted
pub fn pick_number(
    low: u32,
    high: u32,
) -> u32 {
    let mut stdout = stdout();
    loop {
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
    let mut count = 1;
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
    if (p1.row as i32 - p0.row as i32).abs() < (p1.col as i32 - p0.col as i32).abs() {
        if p0.col > p1.col {
            vec_between_points_low(p1, p0).into_iter().rev().collect()
        } else {
            vec_between_points_low(p0, p1)
        }
    } else if p0.row > p1.row {
        vec_between_points_high(p1, p0).into_iter().rev().collect()
    } else {
        vec_between_points_high(p0, p1)
    }
}

fn vec_between_points_low(
    p0: &Point,
    p1: &Point,
) -> Vec<Point> {
    let dx:i32 = p1.col as i32 - p0.col as i32;
    let mut dy: i32 = p1.row as i32 - p0.row as i32;
    let mut yi: i32 = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut diff = (2 * dy) - dx;
    let mut y: i32 = p0.row as i32;

    let mut line: Vec<Point> = Vec::new();
    for x in p0.col..=p1.col {
        line.push(Point { col: x, row: y as usize});
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
    let mut dx: i32 = p1.col as i32 - p0.col as i32;
    let dy: i32 = p1.row as i32 - p0.row as i32;
    let mut xi: i32 = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut diff:i32 = (2 * dx) - dy;
    let mut x:i32 = p0.col as i32;

    let mut line: Vec<Point> = Vec::new();
    for y in p0.row..=p1.row {
        line.push(Point { col:x as usize, row: y });
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
    return input
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix("\n"))
        .unwrap_or(&input);
}

//Generate a pretty timestamp
// Taken from Rosetta Code: https://rosettacode.org/wiki/Convert_seconds_to_compound_duration#Rust
pub struct CompoundTime {
    d: u32,
    h: u32,
    m: u32,
    s: u32,
}

macro_rules! reduce {
    ($s: ident, $(($from: ident, $to: ident, $factor: expr)),+) => {{
        $(
            $s.$to += $s.$from / $factor;
            $s.$from %= $factor;
        )+
    }}
}

impl CompoundTime {
    #[inline]
    pub fn new(seconds: u32) -> Self {
        let mut ct = CompoundTime {
            d: 0,
            h: 0,
            m: 0,
            s: seconds,
        };
        ct.balance();
        ct
    }

    #[inline]
    pub fn balance(&mut self) {
        reduce!(self, (s, m, 60), (m, h, 60), (h, d, 24));
    }
}

impl fmt::Display for CompoundTime {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        //Round seconds to nearet 10 to make it less noisy
        write!(
            f,
            "Day {:02} Time: {:02}:{:02}:{:02}",
            self.d,
            self.h,
            self.m,
            self.s - self.s % 10
        )
    }
}
