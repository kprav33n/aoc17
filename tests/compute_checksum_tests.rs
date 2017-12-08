extern crate aoc17;

use aoc17::compute_checksum;

#[test]
fn given_example() {
    let input = "5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8";
    assert_eq!(compute_checksum(input), 18);
}
