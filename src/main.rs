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
        _ => println!("Unknown command"),
    }
}
