extern crate aoc17;

use aoc17::day10::*;

#[test]
fn test_knot_product() {
    assert_eq!(knot_product(5, vec![3, 4, 1, 5]), 12);
}
