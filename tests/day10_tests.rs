extern crate aoc17;

use aoc17::day10::*;

#[test]
fn test_knot_product() {
    assert_eq!(knot_product(5, vec![3, 4, 1, 5]), 12);
}

#[test]
fn test_knot_hash() {
    assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
