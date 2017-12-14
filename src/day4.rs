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
