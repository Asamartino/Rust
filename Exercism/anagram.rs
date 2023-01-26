// Instructions
// An anagram is a rearrangement of letters to form a new word. Given a word and a list of candidates, select the sublist of anagrams of the given word.

// Given "listen" and a list of candidates like "enlists" "google" "inlets" "banana" the program should return a list containing "inlets".

// The solution is case insensitive, which means "WOrd" is the same as "word" or "woRd". It may help to take a peek at the std library for functions that can convert between them.

// The solution cannot contain the input word. A word is always an anagram of itself, which means it is not an interesting result. Given "hello" and the list ["hello", "olleh"] the answer is ["olleh"].

// You are going to have to adjust the function signature provided in the stub in order for the lifetimes to work out properly.
// This is intentional: what's there demonstrates the basics of lifetime syntax, and what's missing teaches how to interpret lifetime-related compiler errors.

// Try to limit case changes. Case changes are expensive in terms of time, so it's faster to minimize them.

// If sorting, consider sort_unstable which is typically faster than stable sorting. When applicable, unstable sorting is preferred because it is generally faster than stable sorting and
// it doesn't allocate auxiliary memory.

use std::collections::HashSet;

fn to_lowercase_and_sort(word: &str) -> Vec<char> {
    let mut vec: Vec<char> = word.to_lowercase().chars().collect();
    vec.sort();
    vec
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = to_lowercase_and_sort(word);
    let mut response: HashSet<&str> = HashSet::new();

    for i in 0..possible_anagrams.len() {
        let sorted_possible_anagram = to_lowercase_and_sort(possible_anagrams[i]);

        if sorted_word == sorted_possible_anagram
            && word.to_lowercase() != possible_anagrams[i].to_lowercase()
        {
            response.insert(possible_anagrams[i]);
        }
    }
    
    response
}
