// Instructions
// Determine if a sentence is a pangram. A pangram (Greek: παν γράμμα, pan gramma, "every letter") is a sentence using every letter of the alphabet at least once. The best known English pangram is:

// The quick brown fox jumps over the lazy dog.

// The alphabet used consists of ASCII letters a to z, inclusive, and is case insensitive. Any characters which are not an ASCII letter should be ignored.

// Determine whether a sentence is a pangram.

use std::collections::HashMap;

pub fn is_pangram(sentence: &str) -> bool {
    let sentence = sentence.to_ascii_lowercase();  
    let mut map: HashMap<char, u32> = ('a'..='z').map(|x| (x, 0)).collect();    // could aslo simply do ('a'..='z').all(|c| sentence.contains(c)) -> it's an overkill to use a hashmap 

    for char in sentence.chars(){
        if char >= 'a' || char <= 'z'{
            *map.entry(char).or_insert(0) += 1;
        }
    }
    map.values().all(|&x| x > 0)
    
}
