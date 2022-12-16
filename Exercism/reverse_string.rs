// Instructions
// Reverse a string

// For example: input: "cool" output: "looc"

// Test your function on this string: uüu and see what happens. Try to write a function that properly reverses this string. Hint: grapheme clusters

// To get the bonus test to run, remove the ignore flag (#[ignore]) from the last test, and execute the tests with:

// $ cargo test --features grapheme
// You will need to use external libraries (a crate in rust lingo) for the bonus task. A good place to look for those is crates.io, the official repository of crates.

// Check the documentation for instructions on how to use external crates in your projects.

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let s: String = input
        // Split the string into an Iterator of &strs, where each element is an extended grapheme cluster.
        .graphemes(true)
        // Reverse the order of the grapheme iterator.
        .rev()
        // Collect all the chars into a new owned String.
        .collect();
    return s;
}

pub fn reverse2(input: &str) -> String {
    let s = input.as_bytes().iter().rev();

    return s;
}

fn main() {
    let s = String::from("abcdef");
    println!("{}", reverse(&s));
    println!("{}", reverse2(&s));
}
