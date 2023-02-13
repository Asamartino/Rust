// Instructions
// Given a DNA strand, return its RNA complement (per RNA transcription).

// Both DNA and RNA strands are a sequence of nucleotides.

// The four nucleotides found in DNA are adenine (A), cytosine (C), guanine (G) and thymine (T).

// The four nucleotides found in RNA are adenine (A), cytosine (C), guanine (G) and uracil (U).

// Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:

// G -> C
// C -> G
// T -> A
// A -> U
// By using private fields in structs with public new functions returning Option or Result (as here with DNA::new & RNA::new), we can guarantee that the internal representation of DNA is correct.
// Because every valid DNA string has a valid RNA string, we don't need to return a Result/Option from into_rna.

// This explains the type signatures you will see in the tests.

const NUCLEOTIDES_DNA: [char; 4] = ['A', 'C', 'G', 'T'];
const NUCLEOTIDES_RNA: [char; 4] = ['U', 'G', 'C', 'A'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (index, char) in dna.chars().enumerate() {
            if !NUCLEOTIDES_DNA.contains(&char) {
                return Err(index);
            }
        }
        Ok(Dna {
            sequence: String::from(dna),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna_string = self
            .sequence
            .chars()
            .map(|nucleotide| match nucleotide {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => unreachable!(),
            })
            .collect();
        Rna {
            sequence: rna_string,
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (index, char) in rna.chars().enumerate() {
            if !NUCLEOTIDES_RNA.contains(&char) {
                return Err(index);
            }
        }
        Ok(Rna {
            sequence: String::from(rna),
        })
    }
}
