extern crate aoc17;

use aoc17::day15::*;

#[test]
fn test_num_match_duel() {
    assert_eq!(num_match_duel(65, 8921, 5, false), 1);
    // assert_eq!(num_match_duel(65, 8921, 40000000), 588);
}

#[test]
fn test_num_match_duel_slow() {
    assert_eq!(num_match_duel(65, 8921, 1054, true), 0);
    assert_eq!(num_match_duel(65, 8921, 1055, true), 1);
    // assert_eq!(num_match_duel(65, 8921, 5000000, true), 309);
}
