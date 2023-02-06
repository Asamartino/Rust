// Instructions
// Determine if a word or phrase is an isogram.

// An isogram (also known as a "nonpattern word") is a word or phrase without a repeating letter, however spaces and hyphens are allowed to appear multiple times.

// Examples of isograms:

// lumberjacks
// background
// downstream
// six-year-old
// The word isograms, however, is not an isogram, because the s repeats

use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    // data cleaning
    let candidate = candidate.replace(&['-', ' '][..], "");
    let candidate = candidate.to_ascii_lowercase();

    let mut hashmap_char_value = HashMap::new();

    for (i, char) in candidate.chars().enumerate() {
        if !hashmap_char_value.contains_key(&char) {
            hashmap_char_value.insert(char, i);
        } else {
            return false;
        }
    }
    true
}
