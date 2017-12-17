extern crate aoc17;
use std::env;
use std::io::{self, Read};

fn second_argument() -> String {
    env::args().nth(2).unwrap()
}

fn report_result_using_second_arg_str<T: std::fmt::Display>(f: fn(&str) -> T) {
    println!("{}", f(&second_argument() as &str));
}

fn read_stdin_and_report_result<T: std::fmt::Display>(f: fn(&str) -> T) {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => println!("{}", f(&buffer.trim())),
        Err(e) => println!("Failed to read from STDIN: {}", e),
    }
}

fn report_result_using_second_arg<T: std::str::FromStr + std::fmt::Display>(f: fn(T) -> T) {
    let arg = second_argument();
    match arg.parse::<T>() {
        Ok(v) => println!("{}", f(v)),
        Err(_) => println!("Failed to parse argument {}", arg),
    }
}

fn main() {
    let command = &env::args().nth(1).unwrap() as &str;
    match command {
        "solve-captcha" => report_result_using_second_arg_str(
            aoc17::day01::solve_captcha
        ),
        "solve-captcha2" => report_result_using_second_arg_str(
            aoc17::day01::solve_captcha2
        ),
        "compute-checksum" => read_stdin_and_report_result(
            aoc17::day02::compute_checksum
        ),
        "compute-checksum2" => read_stdin_and_report_result(
            aoc17::day02::compute_checksum2
        ),
        "manhattan-distance" => report_result_using_second_arg(
            aoc17::day03::manhattan_distance
        ),
        "next-in-sum-spiral" => report_result_using_second_arg(
            aoc17::day03::next_in_sum_spiral
        ),
        "count-valid-passphrases" => read_stdin_and_report_result(
            aoc17::day04::count_valid_passphrases
        ),
        "count-valid-passphrases-secure" => read_stdin_and_report_result(
            aoc17::day04::count_valid_passphrases_secure
        ),
        "steps-until-exit" => read_stdin_and_report_result(
            |x| aoc17::day05::steps_until_exit(x, false)
        ),
        "steps-until-exit-strange" => read_stdin_and_report_result(
            |x| aoc17::day05::steps_until_exit(x, true)
        ),
        "num-redist" => read_stdin_and_report_result(
            |x| aoc17::day06::num_redist(x).0
        ),
        "num-redist2" => read_stdin_and_report_result(
            |x| aoc17::day06::num_redist(x).1
        ),
        "num-match-duel" => read_stdin_and_report_result(
            |x| {
                let v: Vec<u32> = x.split_whitespace().map(|i| i.parse::<u32>().unwrap()).collect();
                aoc17::day15::num_match_duel(v[0], v[1], 40000000, false)
            }
        ),
        "num-match-duel2" => read_stdin_and_report_result(
            |x| {
                let v: Vec<u32> = x.split_whitespace().map(|i| i.parse::<u32>().unwrap()).collect();
                aoc17::day15::num_match_duel(v[0], v[1], 5000000, true)
            }
        ),
        "knot-product" => read_stdin_and_report_result(
            |x| {
                let v = x.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                aoc17::day10::knot_product(256, v)
            }
        ),
        "knot-hash" => read_stdin_and_report_result(
            |x| {
                aoc17::day10::knot_hash(x.trim())
            }
        ),
        "perm-promenade" => read_stdin_and_report_result(
            |x| {
                aoc17::day16::perm_promenade("abcdefghijklmnop", x.trim(), 1)
            }
        ),
        "perm-promenade-whole" => read_stdin_and_report_result(
            |x| {
                aoc17::day16::perm_promenade("abcdefghijklmnop", x.trim(), 100)
            }
        ),
        "num-used-grids" => read_stdin_and_report_result(
            aoc17::day14::num_used_grids
        ),
        "num-used-regions" => read_stdin_and_report_result(
            aoc17::day14::num_used_regions
        ),
        _ => println!("Unknown command: {}", command),
    }
}
