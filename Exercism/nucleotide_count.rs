// Instructions
// Each of us inherits from our biological parents a set of chemical instructions known as DNA that influence how our bodies are constructed. All known life depends on DNA!

// Note: You do not need to understand anything about nucleotides or DNA to complete this exercise.

// DNA is a long chain of other chemicals and the most important are the four nucleotides, adenine, cytosine, guanine and thymine. A single DNA chain can contain billions of 
// these four nucleotides and the order in which they occur is important! We call the order of these nucleotides in a bit of DNA a "DNA sequence".

// We represent a DNA sequence as an ordered collection of these four nucleotides and a common way to do that is with a string of characters such as "ATTACG" for a DNA sequence of 6 nucleotides. 
// 'A' for adenine, 'C' for cytosine, 'G' for guanine, and 'T' for thymine.

// Given a string representing a DNA sequence, count how many of each nucleotide is present. If the string contains characters that aren't A, C, G, or T then it is invalid and you should signal an error.

// For example:

// "GATTACA" -> 'A': 3, 'C': 1, 'G': 1, 'T': 2
// "INVALID" -> error


use std::collections::HashMap;

const NUCLEOTIDES: [char;4] = ['A', 'C', 'G', 'T'];

pub fn is_nucleotide(char: char) -> bool {
    if NUCLEOTIDES.contains(&char) {
        return true;
    }
    false
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }
    let mut counter = 0;
    for char in dna.chars() {
        if char == nucleotide {
            counter += 1;
        }
        if !is_nucleotide(char) {
            return Err(char);
        }
    }
    Ok(counter)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hm_result: HashMap<_, _> = NUCLEOTIDES.iter().map(|&x| (x, 0)).collect(); // need to be initialize due to some test

    for char in dna.chars() {
        if is_nucleotide(char) {
            *hm_result.entry(char).or_default() += 1;
        } else {
            return Err(char);
        }
    }
    Ok(hm_result)
}
