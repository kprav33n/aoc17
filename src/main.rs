extern crate aoc17;
use std::env;

fn main() {
    let command = &env::args().nth(1).unwrap() as &str;
    match command {
        "solve-captcha" => println!("{}", aoc17::solve_captcha(&env::args().nth(2).unwrap() as &str)),
        "solve-captcha2" => println!("{}", aoc17::solve_captcha2(&env::args().nth(2).unwrap() as &str)),
        _ => println!("Unknown command"),
    }
}