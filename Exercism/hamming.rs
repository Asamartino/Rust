// Instructions
// Calculate the Hamming Distance between two DNA strands.

// Your body is made up of cells that contain DNA. Those cells regularly wear out and need replacing, which they achieve by dividing into daughter cells. 
// In fact, the average human body experiences about 10 quadrillion cell divisions in a lifetime!

// When cells divide, their DNA replicates too. Sometimes during this process mistakes happen and single pieces of DNA get encoded with the incorrect information. 
// If we compare two strands of DNA and count the differences between them we can see how many mistakes occurred. This is known as the "Hamming Distance".

// We read DNA using the letters C,A,G and T. Two strands might look like this:

// GAGCCTACTAACGGGAT
// CATCGTAATGACGGCCT
// ^ ^ ^  ^ ^    ^^
// They have 7 differences, and therefore the Hamming Distance is 7.

// The Hamming Distance is useful for lots of things in science, not just biology, so it's a nice phrase to be familiar with :)

// The Hamming distance is only defined for sequences of equal length, so an attempt to calculate it between sequences of different lengths should not work. 
// The general handling of this situation (e.g., raising an exception vs returning a special value) may differ between languages.


// Return the Hamming distance between the strings,
// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut difference = 0;

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            difference += 1;
        }
    }
    return Some(difference);
}
