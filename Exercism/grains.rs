// Instructions
// Calculate the number of grains of wheat on a chessboard given that the number on each square doubles.

// There once was a wise servant who saved the life of a prince. The king promised to pay whatever the servant could dream up. Knowing that the king loved chess, the servant told the king he would like to have grains of wheat. One grain on the first square of a chess board, with the number of grains doubling on each successive square.

// There are 64 squares on a chessboard (where square 1 has one grain, square 2 has two grains, and so on).

// Write code that shows:

// how many grains were on a given square,
// the total number of grains on the chessboard, and
// panics with a message of "Square must be between 1 and 64" if the value is not valid
// For bonus points
// Did you get the tests passing and the code clean? If you want to, these are some additional things you could try:

// Optimize for speed.
// Optimize for readability.
// Then please share your thoughts in a comment on the submission. Did this experiment make the code better? Worse? Did you learn anything from it?

use num_traits::pow;

pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    pow(2 as u64, (s - 1) as usize)
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for i in 0..64 {
        sum += pow(2 as u64, i);
    }
    sum
}
