extern crate aoc17;
use std::env;

fn main() {
    let command = &env::args().nth(1).unwrap() as &str;
    match command {
        "solve-captcha" => println!("{}", aoc17::solve_captcha(&env::args().nth(2).unwrap() as &str)),
        _ => println!("Unknown command"),
    }
}