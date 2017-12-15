extern crate aoc17;

use aoc17::day06::*;

#[test]
fn test_num_redist() {
    assert_eq!(num_redist("0\t2\t7\t0"), (5, 4));
}
