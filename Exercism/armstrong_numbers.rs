// Instructions
// An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.

// For example:

// 9 is an Armstrong number, because 9 = 9^1 = 9
// 10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
// 153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
// 154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
// Write some code to determine whether a number is an Armstrong number.

use num_traits::pow;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut n = num;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);

    let mut total = 0;
    let mut sum = 0;
    for i in 0..digits.len() {
        sum += pow(digits[i], digits.len()) as u64;
        total += (digits[i] * pow(10, i)) as u64;
    }
    if sum == total {
        return true;
    }
    false
}

fn main() {
    println!(
        "is it a armstrong number {:?}",
        is_armstrong_number(4_106_098_957)
    )
}
