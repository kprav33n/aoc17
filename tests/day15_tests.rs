extern crate aoc17;

use aoc17::day15::*;

#[test]
fn test_num_match_duel() {
    assert_eq!(num_match_duel(65, 8921, 5), 1);
    assert_eq!(num_match_duel(65, 8921, 40000000), 588);
}
