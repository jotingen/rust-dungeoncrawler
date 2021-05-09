use rand::Rng;
use regex::Regex;

fn main() {
    println!("Hello, world!");

    //Stats
    let mut rolls: [u8; 6] = [15, 14, 13, 12, 10, 8];
    println!("Default stats are {:?}", rolls);
    let mut roll_my_own = String::new();
    println!("Roll your own stats? Y/n");
    std::io::stdin().read_line(&mut roll_my_own).unwrap();
    let re_yes = Regex::new(r"^\s*[yY]\s*$").unwrap();
    if re_yes.is_match(&roll_my_own) {
        roll_stats(&mut rolls);
    }
    println!("Using stats {:?}", rolls);
}

fn roll_stats(rolls: &mut [u8; 6]) {
    let mut rng = rand::thread_rng();
    println!("Rolling own");
    for roll in rolls.iter_mut() {
        let mut die_rolls: [u8; 4] = [
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
        ];
        die_rolls.sort_unstable();
        die_rolls.reverse();
        *roll = die_rolls[0] + die_rolls[1] + die_rolls[2];
        println!("Rolled {:?} for {}", die_rolls, *roll);
    }
    rolls.sort_unstable();
    rolls.reverse();
}
