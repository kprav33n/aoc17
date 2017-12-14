pub fn is_valid_passphrase(input: &str) -> bool {
    use std::collections::HashSet;
    let mut words: HashSet<&str> = HashSet::new();
    for word in input.split_whitespace() {
        if words.contains(word) {
            return false;
        }
        words.insert(word);
    }
    true
}

pub fn count_valid_passphrases(input: &str) -> usize {
    input.lines().map(is_valid_passphrase).fold(0, |count, v| if v { count + 1 } else { count })
}

pub fn is_valid_passphrase_secure(input: &str) -> bool {
    use std::collections::HashSet;
    let mut words: HashSet<String> = HashSet::new();
    for word in input.split_whitespace() {
        // FIXME: Yikes! Is there a simpler way to sort a word?
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let sorted = chars.into_iter().collect();
        if words.contains(&sorted) {
                return false;
        }
        words.insert(sorted);
    }
    true
}

pub fn count_valid_passphrases_secure(input: &str) -> usize {
    input.lines().map(is_valid_passphrase_secure).fold(0, |count, v| if v { count + 1 } else { count })
}
