extern crate aoc17;

use aoc17::day16::*;

#[test]
fn test_perm_promenade() {
    assert_eq!(perm_promenade("abcde", "s1,x3/4,pe/b", 1), "baedc");
    assert_eq!(perm_promenade("abcde", "s1,x3/4,pe/b", 2), "ceadb");
}
