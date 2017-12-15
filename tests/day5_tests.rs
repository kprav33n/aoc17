extern crate aoc17;

use aoc17::day5::steps_until_exit;

#[test]
fn test_example() {
    assert_eq!(steps_until_exit("0\n3\n0\n1\n-3"), 5);
}
