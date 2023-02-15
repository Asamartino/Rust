// Instructions
// Given two lists determine if the first list is contained within the second list, if the second list is contained within the first list, if both lists are contained within each other or if none of these are true.

// Specifically, a list A is a sublist of list B if by dropping 0 or more elements from the front of B and 0 or more elements from the back of B you get a list that's completely equal to A.

// Examples:

// A = [1, 2, 3], B = [1, 2, 3, 4, 5], A is a sublist of B
// A = [3, 4, 5], B = [1, 2, 3, 4, 5], A is a sublist of B
// A = [3, 4], B = [1, 2, 3, 4, 5], A is a sublist of B
// A = [1, 2, 3], B = [1, 2, 3], A is equal to B
// A = [1, 2, 3, 4, 5], B = [2, 3, 4], A is a superlist of B
// A = [1, 2, 4], B = [1, 2, 3, 4, 5], A is not a superlist of, sublist of or equal to B

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}
pub fn sublist<T: Eq>(first_array: &[T], second_array: &[T]) -> Comparison {
    use Comparison::*;
    match (first_array.len(), second_array.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (i, j) if i > j => {
            if first_array
                .windows(j)
                .any(|sublist| sublist == second_array)
            {
                Superlist
            } else {
                Unequal
            }
        }
        (i, j) if i < j => {
            if second_array
                .windows(i)
                .any(|sublist| sublist == first_array)
            {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if first_array == second_array {
                Equal
            } else {
                Unequal
            }
        }
    }
}
