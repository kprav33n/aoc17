extern crate aoc17;

use aoc17::day4::*;

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

#[test]
fn test_is_valid_passphrase_secure() {
    assert!(is_valid_passphrase_secure("abcde fghij"));
    assert!(!is_valid_passphrase_secure("abcde xyz ecdab"));
    assert!(is_valid_passphrase_secure("a ab abc abd abf abj"));
    assert!(is_valid_passphrase_secure("iiii oiii ooii oooi oooo"));
    assert!(!is_valid_passphrase_secure("oiii ioii iioi iiio"));
}

#[test]
fn test_count_valid_passphrases_secure() {
    assert_eq!(count_valid_passphrases_secure("abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio\n"), 3);
}
