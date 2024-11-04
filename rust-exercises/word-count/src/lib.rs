use std::collections::HashMap;

/// Count occurrences of words in a case-insensitive manner.
pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in phrase.split_whitespace() {
        let word = word.to_lowercase(); // Normalize to lowercase
        *counts.entry(word).or_insert(0) += 1; // Increment count
    }
    counts
}
