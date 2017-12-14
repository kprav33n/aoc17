extern crate aoc17;

use aoc17::manhattan_distance;

#[test]
fn test_access_port() {
    assert_eq!(manhattan_distance(1), 0);
}

#[test]
fn test_square_12() {
    assert_eq!(manhattan_distance(12), 3);
}

#[test]
fn test_square_23() {
    assert_eq!(manhattan_distance(23), 2);
}

#[test]
fn test_square_1024() {
    assert_eq!(manhattan_distance(1024), 31);
}
