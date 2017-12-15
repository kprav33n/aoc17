extern crate aoc17;

use aoc17::day02::*;

#[test]
fn test_compute_checksum() {
    let input = "5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8";
    assert_eq!(compute_checksum(input), 18);
}

#[test]
fn test_compute_checksum2() {
    assert_eq!(compute_checksum2("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5"), 9);
}
