extern crate aoc17;

use aoc17::day5::*;

#[test]
fn test_steps_until_exit() {
    assert_eq!(steps_until_exit("0\n3\n0\n1\n-3", false), 5);
}

#[test]
fn test_steps_until_exit_strange() {
    assert_eq!(steps_until_exit("0\n3\n0\n1\n-3", true), 10);
}
