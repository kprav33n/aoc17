extern crate aoc17;

use aoc17::solve_captcha2;

#[test]
fn all_match() {
    assert_eq!(solve_captcha2("1212"), 6);
    assert_eq!(solve_captcha2("123123"), 12);
}

#[test]
fn none_match() {
    assert_eq!(solve_captcha2("1221"), 0);
}

#[test]
fn part_mach() {
    assert_eq!(solve_captcha2("123425"), 4);
    assert_eq!(solve_captcha2("12131415"), 4);
}
