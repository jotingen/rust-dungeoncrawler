use rand::Rng;
use regex::Regex;
use std::io::{stdin, stdout, Write};
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
