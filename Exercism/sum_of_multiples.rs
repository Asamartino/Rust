// Instructions
// Given a number, find the sum of all the unique multiples of particular numbers up to but not including that number.

// If we list all the natural numbers below 20 that are multiples of 3 or 5, we get 3, 5, 6, 9, 10, 12, 15, and 18.

// The sum of these multiples is 78.

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut all_natural_mutiples = Vec::new();
    for factor in factors {
        if factor == &0 {
            continue;
        }
        for i in 1..(limit / factor + 1) {
            if factor * i < limit {
                if !all_natural_mutiples.contains(&(factor * &i)) {
                    all_natural_mutiples.push(factor * &i);
                }
            }
        }
    }
    let sum = all_natural_mutiples.iter().sum();
    return sum;
}

// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     let mut all_natural_mutiples = Vec::new();
//     for factor in factors {
//         if factor == &0{
//             continue;
//         }
//         for i in 1..(limit / factor + 1) {
//             if factor * i < limit {
//                 all_natural_mutiples.push(factor * &i);
//             }
//         }
//     }
//     // let sum = all_natural_mutiples.sort().dedup().iter().sum();
//     all_natural_mutiples.sort();
//     all_natural_mutiples.dedup();
//     let sum = all_natural_mutiples.iter().sum();
//     return sum;
// }

fn main() {
    println!("Hello world");
}
