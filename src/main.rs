extern crate aoc17;
use std::env;
use std::io::{self, Read};

fn main() {
    let command = &env::args().nth(1).unwrap() as &str;
    match command {
        "solve-captcha" => println!("{}", aoc17::solve_captcha(&env::args().nth(2).unwrap() as &str)),
        "solve-captcha2" => println!("{}", aoc17::solve_captcha2(&env::args().nth(2).unwrap() as &str)),
        "compute-checksum" => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => println!("{}", aoc17::compute_checksum(&buffer)),
                Err(e) => println!("Failed to read from STDIN: {}", e),
            }
        }
        "compute-checksum2" => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => println!("{}", aoc17::compute_checksum2(&buffer)),
                Err(e) => println!("Failed to read from STDIN: {}", e),
            }
        }
        "manhattan-distance" => println!("{}", aoc17::manhattan_distance(
            env::args().nth(2).unwrap().parse::<u64>().unwrap()
        )),
        "next-in-sum-spiral" => println!("{}", aoc17::next_in_sum_spiral(
            env::args().nth(2).unwrap().parse::<i64>().unwrap()
        )),
        "count-valid-passphrases" => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => println!("{}", aoc17::day4::count_valid_passphrases(&buffer)),
                Err(e) => println!("Failed to read from STDIN: {}", e),
            }
        }
        "count-valid-passphrases-secure" => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => println!("{}", aoc17::day4::count_valid_passphrases_secure(&buffer)),
                Err(e) => println!("Failed to read from STDIN: {}", e),
            }
        }
        "steps-until-exit" => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => println!("{}", aoc17::day5::steps_until_exit(&buffer, false)),
                Err(e) => println!("Failed to read from STDIN: {}", e),
            }
        }
        "steps-until-exit-strange" => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => println!("{}", aoc17::day5::steps_until_exit(&buffer, true)),
                Err(e) => println!("Failed to read from STDIN: {}", e),
            }
        }
        _ => println!("Unknown command: {}", command),
    }
}
