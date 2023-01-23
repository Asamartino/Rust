// Instructions
// Convert a number, represented as a sequence of digits in one base, to any other base.

// Implement general base conversion. Given a number in base a, represented as a sequence of digits, convert it to base b.

// Note
// Try to implement the conversion yourself. Do not use something else to perform the conversion for you.
// About Positional Notation
// In positional notation, a number in base b can be understood as a linear combination of powers of b.

// The number 42, in base 10, means:

// (4 * 10^1) + (2 * 10^0)

// The number 101010, in base 2, means:

// (1 * 2^5) + (0 * 2^4) + (1 * 2^3) + (0 * 2^2) + (1 * 2^1) + (0 * 2^0)

// The number 1120, in base 3, means:

// (1 * 3^3) + (1 * 3^2) + (2 * 3^1) + (0 * 3^0)

// I think you got the idea!

// Yes. Those three numbers above are exactly the same. Congratulations!

use num_traits::pow;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    }
    if number.len() == 0 || number.iter().all(|&x| x == 0) {
        return Ok(vec![0]);
    }
    if number.iter().any(|&x| x == from_base) {
        return Err(Error::InvalidDigit(from_base));
    }

    let mut vec_result = Vec::new();
    let mut result_base_10 = 0;

    //convert number "from_base" to base 10
    for i in 0..number.len() {
        result_base_10 += number[i] * pow(from_base, number.len() - 1 - i);
    }
    // convert result_base_10 to  number "to_base"
    while result_base_10 > 0 {
        vec_result.push(result_base_10 % to_base);
        result_base_10 = result_base_10 / to_base;
    }
    vec_result.reverse();
    return Ok(vec_result);
}
