use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::Rng;
use regex::Regex;
use std::io::{self, stdin, stdout, Read, Write};
use unicode_segmentation::UnicodeSegmentation;

pub fn d(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    rng.gen_range(1..(num + 1))
}

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}

pub fn pause() {
    let mut stdout = stdout();
    print!("Press Enter to continue...");
    stdout.flush().unwrap();
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).unwrap();
}

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

//TODO Make this a seperate thread, instead of stalling the game for input
#[allow(clippy::never_loop)]
pub fn enter_char(msg: &str) -> char {
    let mut stdout = stdout();
    if !msg.is_empty() {
        print!("{}", msg);
    }
    stdout.flush().unwrap();
    enable_raw_mode().unwrap();
    let mut my_char: char = ' ';
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

pub fn pick_number(msg: &str, low: u32, high: u32) -> u32 {
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

pub fn count_newlines(msg: &str) -> u32 {
    let mut count = 0;
    for c in msg.graphemes(true) {
        if c == "\n" {
            count += 1;
        }
    }
    count
}

pub fn vec_between_points_low(x0: i32, y0: i32, x1: i32, y1:i32) -> Vec<(i32, i32)> {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut diff = (2*dy)-dx;
    let mut y = y0;

    let mut line: Vec<(i32,i32)> = Vec::new();
    for x in x0..=x1 {
        line.push((x,y));
        if diff>0 {
            y += yi;
            diff += 2*(dy-dx);
        } else {
            diff += 2*dy;
        }
    }
    line
}

pub fn vec_between_points_high(x0: i32, y0: i32, x1: i32, y1:i32) -> Vec<(i32, i32)> {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut diff = (2*dx)-dy;
    let mut x = x0;

    let mut line: Vec<(i32,i32)> = Vec::new();
    for y in y0..=y1 {
        line.push((x,y));
        if diff>0 {
            x += xi;
            diff += 2*(dx-dy);
        } else {
            diff += 2*dx;
        }
    }
    line
}

pub fn vec_between_points(x0: i32, y0: i32, x1: i32, y1:i32) -> Vec<(i32, i32)> {
    if (y1-y0).abs() < (x1-x0).abs() {
        if x0 > x1 {
            vec_between_points_low(x1,y1,x0,y0).into_iter().rev().collect()
        } else {
            vec_between_points_low(x0,y0,x1,y1)
        }
    } else if y0 > y1 {
        vec_between_points_high(x1,y1,x0,y0).into_iter().rev().collect()
    } else {
        vec_between_points_high(x0,y0,x1,y1)
    }
}