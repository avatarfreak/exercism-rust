use std::collections::BTreeSet;
use std::ascii::AsciiExt;

pub fn is_pangram(words: &str) -> bool {
    words.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .filter(|c| c.is_ascii())
        .collect::<BTreeSet<char>>() == letter()
}

fn letter() -> BTreeSet<char> {
    let letter = "abcdefghijklmnopqrstuvwxyz";
    letter.chars().collect::<BTreeSet<_>>()
}

