// Instructions
// Compute the prime factors of a given natural number.

// A prime number is only evenly divisible by itself and 1.

// Note that 1 is not a prime number.

// Example
// What are the prime factors of 60?

// Our first divisor is 2. 2 goes into 60, leaving 30.
// 2 goes into 30, leaving 15.
// 2 doesn't go cleanly into 15. So let's move on to our next divisor, 3.
// 3 goes cleanly into 15, leaving 5.
// 3 does not go cleanly into 5. The next possible factor is 4.
// 4 does not go cleanly into 5. The next possible factor is 5.
// 5 does go cleanly into 5.
// We're left only with 1, so now, we're done.
// Our successful divisors in that computation represent the list of prime factors of 60: 2, 2, 3, and 5.

// You can check this yourself:

// 2 * 2 * 3 * 5
// = 4 * 15
// = 60
// Success!

pub fn factors(n: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    let mut reduced_number = n;
    let mut i = 2;
    while reduced_number > 0{
        if reduced_number == 1 {
            break;
        }
        if reduced_number % i == 0 || reduced_number == i {
            vec.push(i);
            reduced_number = reduced_number/i;
        }
        if reduced_number % i != 0{
             i += 1;
        }
    }
    return vec;
}


fn main() {
    println!("the prime factors of 60 is {:?}", factors(60));
}
