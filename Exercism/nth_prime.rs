// Instructions
// Given a number n, determine what the nth prime is.

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// If your language provides methods in the standard library to deal with prime numbers, pretend they don't exist and implement them yourself.

// Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages,
// including Rust, use 0-based indexing (i.e. primes[5] == 13). Use 0-based indexing for your implementation.


pub fn nth(n: u32) -> u32 {
        let mut counter = 1;
        let mut last_prime_number = 0;
        if n == 0 {
            return 2;
        }
        if n == 1 {
            return 3;
        }
        let maxu32 = u32::MAX;
        for i in 4..maxu32 {
                if counter == n as usize {
                    break;
                }
                let mut j = i - 1;
                while j > 0 {
                    if j == 1 {
                        last_prime_number = i;
                        counter += 1
                    }
                    if i % j == 0 {
                        break;
                    }
                    j -= 1;
                }
            }
        return last_prime_number;
}

// using a vector to collect all prime numbers 
// pub fn nth(n: u32) -> u32 {
//     let mut indexed_prime_number = vec![2, 3];
//     if n == 0 {
//         return indexed_prime_number[0];
//     }
//     if n == 1 {
//         return indexed_prime_number[1];
//     }
//     let maxu64 = u32::MAX;
//     for i in 4..maxu64 {
//             if indexed_prime_number.len() == n as usize {
//                 break
//             }
//             let mut j = i - 1;
//             while j > 0 {
//                 if j == 1 {
//                     indexed_prime_number.push(i);
//                 }
//                 if i % j == 0 {
//                     break;
//                 }
//                 j -= 1;
//             }
//         }
//     return indexed_prime_number[(n - 1) as usize];
// }

fn main() {
    let n = 10_000;
    println!(
        "The 0-based indexing (i.e. primes[5] == 13) of the number {} is {}",
        n,
        nth(n)
    );
}
