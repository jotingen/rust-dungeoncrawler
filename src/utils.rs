use rand::Rng;
use regex::Regex;
use std::io::{stdin, stdout, Read, Write};

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
    stdin().read_exact(&mut [0]).unwrap();
}

pub fn pick_yes_or_no(msg: &str) -> bool {
    println!("{} Y/n", msg);
    let mut my_yes_or_no_str = String::new();
    stdin().read_line(&mut my_yes_or_no_str).unwrap();

    //regex for empty/y*/Y*
    let re_yes = Regex::new(r"^(?i)\s*y(es)?\s*$").unwrap();
    if re_yes.is_match(&my_yes_or_no_str) || my_yes_or_no_str.trim().is_empty() {
        return true;
    }
    false
}

pub fn pick_number(msg: &str, low: u32, high: u32) -> u32 {
    loop {
        if !msg.is_empty() {
            print!("{} ", msg);
        }
        println!("{}-{}", low, high);
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
