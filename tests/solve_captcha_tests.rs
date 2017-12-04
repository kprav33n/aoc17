extern crate aoc17;

use aoc17::solve_captcha;

#[test]
fn some_match() {
    assert_eq!(solve_captcha("1122"), 3);
}

#[test]
fn all_match() {
    assert_eq!(solve_captcha("1111"), 4);
}

#[test]
fn none_match() {
    assert_eq!(solve_captcha("1234"), 0);
}

#[test]
fn cyclic_match() {
    assert_eq!(solve_captcha("91212129"), 9);
}