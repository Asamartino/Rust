// Instructions
// Given students' names along with the grade that they are in, create a roster for the school.

// In the end, you should be able to:

// Add a student's name to the roster for a grade
// "Add Jim to grade 2."
// "OK."
// Get a list of all students enrolled in a grade
// "Which students are in grade 2?"
// "We've only got Jim just now."
// Get a sorted list of all students in all grades. Grades should sort as 1, 2, 3, etc., and students within a grade should be sorted alphabetically by name.
// "Who all is enrolled in school right now?"
// "Let me think. We have Anna, Barb, and Charlie in grade 1, Alex, Peter, and Zoe in grade 2 and Jim in grade 5. So the answer is: Anna, Barb, Charlie, Alex, Peter, Zoe and Jim"
// Note that all our students only have one name. (It's a small town, what do you want?)

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    grade_students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grade_students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grade_students
            .entry(grade)
            .or_insert(Vec::new())
            .push(student.to_string())
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut keys = vec![];
        if !self.grade_students.is_empty() {
            for key in self.grade_students.keys() {
                keys.push(*key);
            }
        }
        keys.sort();
        keys
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut owned_vector = vec![];
        if self.grade_students.contains_key(&grade) {
            for student in &self.grade_students[&grade] {
                owned_vector.push(student.to_string())
            }
        }
        owned_vector.sort();
        owned_vector
    }
}
