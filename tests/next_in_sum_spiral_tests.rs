extern crate aoc17;

use aoc17::next_in_sum_spiral;

#[test]
fn test_next_of() {
    assert_eq!(next_in_sum_spiral(1), 2);
    assert_eq!(next_in_sum_spiral(2), 4);
    assert_eq!(next_in_sum_spiral(4), 5);
    assert_eq!(next_in_sum_spiral(5), 10);
    assert_eq!(next_in_sum_spiral(747), 806);
    assert_eq!(next_in_sum_spiral(361527), 363010);
}
