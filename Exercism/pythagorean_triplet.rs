// Instructions
// A Pythagorean triplet is a set of three natural numbers, {a, b, c}, for which,

// a**2 + b**2 = c**2
// and such that,

// a < b < c
// For example,

// 3**2 + 4**2 = 9 + 16 = 25 = 5**2.
// Given an input integer N, find all Pythagorean triplets for which a + b + c = N.

// For example, with N = 1000, there is exactly one Pythagorean triplet for which a + b + c = 1000: {200, 375, 425}.

use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut pythagorean_triplet = HashSet::new();
    for i in 1..sum / 2 {
        for j in i + 1..sum / 2 {
            let k = sum - i - j;
            if i * i + j * j == k * k {
                pythagorean_triplet.insert([i, j, k]);
            }
        }
    }
    return pythagorean_triplet;
}
