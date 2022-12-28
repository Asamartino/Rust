// Instructions
// The Collatz Conjecture or 3x+1 problem can be summarized as follows:

// Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1. 
// Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.

// But sometimes the number grow significantly before it reaches 1. This can lead to an integer overflow and makes the algorithm unsolvable within the range of a number in u64.

// Given a number n, return the number of steps required to reach 1.

// Examples
// Starting with n = 12, the steps would be as follows:

// 12
// 6
// 3
// 10
// 5
// 16
// 8
// 4
// 2
// 1
// Resulting in 9 steps. So for input n = 12, the return value would be 9.

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut number = n;
    let mut steps = 0;
    while number > 1 {
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number.checked_mul(3)?.checked_add(1)?; //check that there is no under/overflow by mutiplying by 3 and then adding 1, if under/overflow returns None
            number = number * 3 + 1;
        }
        steps += 1;
    }
    Some(steps)
}
