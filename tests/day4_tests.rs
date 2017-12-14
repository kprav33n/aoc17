extern crate aoc17;

use aoc17::day4::is_valid_passphrase;
use aoc17::day4::count_valid_passphrases;

#[test]
fn test_is_valid_passphrase() {
    assert!(is_valid_passphrase("aa bb cc dd ee"));
    assert!(!is_valid_passphrase("aa bb cc dd aa"));
    assert!(is_valid_passphrase("aa bb cc dd aaa"));
}

#[test]
fn test_count_valid_passphrases() {
    assert_eq!(count_valid_passphrases("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa"), 2);
}
