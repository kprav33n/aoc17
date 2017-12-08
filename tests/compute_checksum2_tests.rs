extern crate aoc17;

use aoc17::compute_checksum2;

#[test]
fn given_example() {
    assert_eq!(compute_checksum2("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5"), 9);
}
