extern crate aoc17;

use aoc17::day03::*;

#[test]
fn test_manhattan_distance() {
    assert_eq!(manhattan_distance(1), 0);
    assert_eq!(manhattan_distance(12), 3);
    assert_eq!(manhattan_distance(23), 2);
    assert_eq!(manhattan_distance(1024), 31);
}

#[test]
fn test_next_in_sum_spiral() {
    assert_eq!(next_in_sum_spiral(1), 2);
    assert_eq!(next_in_sum_spiral(2), 4);
    assert_eq!(next_in_sum_spiral(4), 5);
    assert_eq!(next_in_sum_spiral(5), 10);
    assert_eq!(next_in_sum_spiral(747), 806);
    assert_eq!(next_in_sum_spiral(361527), 363010);
}
