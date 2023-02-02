// Instructions
// Convert a phrase to its acronym.

// Techies love their TLA (Three Letter Acronyms)!

// Help generate some jargon by writing a program that converts a long name like Portable Network Graphics to its acronym (PNG).

pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();

    // data cleaning
    let phrase = phrase.replace(&['-', '_', ':'][..], " ");

    for iter in phrase.split_whitespace() {
        if iter.chars().all(|c| c.is_uppercase()) || iter.chars().all(|c| c.is_lowercase()) {
            result.push(iter.chars().nth(0).unwrap().to_ascii_uppercase())
        } else {
            // complication due to the test "HyperText"
            let uppercase_letters: String = iter.chars().filter(|c| c.is_uppercase()).collect();
            result.push_str(&uppercase_letters);
        }
    }
    result
}
