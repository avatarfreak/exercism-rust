use std::collections::HashMap;

pub fn count(ch: char, words: &str) -> usize {
    words.trim().chars().filter(|c| ch == *c).count()
}

pub fn nucleotide_counts(words: &str) -> HashMap<char, usize> {
    let mut hashmap: HashMap<char, usize> = HashMap::new();
    for ch in "ATCG".chars() {
        hashmap.insert(ch, 0);
    }

    for ch in words.chars() {
        let occured_times = match hashmap.get(&ch) {
            Some(n) => n + 1,
            None => 1,
        };

        hashmap.insert(ch, occured_times);
    }
    hashmap
}
