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
        "is it a arsttrong number {:?}",
        is_armstrong_number(4_106_098_957)
    )
}
