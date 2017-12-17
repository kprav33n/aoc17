extern crate aoc17;

use aoc17::day17::*;

#[test]
fn test_spinlock_end_val() {
    assert_eq!(spinlock_end_val(3, 1), 0);
    assert_eq!(spinlock_end_val(3, 2), 1);
    assert_eq!(spinlock_end_val(3, 3), 1);
    assert_eq!(spinlock_end_val(3, 4), 3);
    assert_eq!(spinlock_end_val(3, 2017), 638)
}
