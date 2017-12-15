extern crate aoc17;

use aoc17::day01::*;

#[test]
fn test_solve_captch() {
    assert_eq!(solve_captcha("1122"), 3);
    assert_eq!(solve_captcha("1111"), 4);
    assert_eq!(solve_captcha("1234"), 0);
    assert_eq!(solve_captcha("91212129"), 9);
}

#[test]
fn test_solve_captcha2() {
    assert_eq!(solve_captcha2("1212"), 6);
    assert_eq!(solve_captcha2("123123"), 12);
    assert_eq!(solve_captcha2("1221"), 0);
    assert_eq!(solve_captcha2("123425"), 4);
    assert_eq!(solve_captcha2("12131415"), 4);
}
