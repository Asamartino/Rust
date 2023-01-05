use rand::{thread_rng, Rng};
use num_bigint::BigUint;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2,p) //needs to be bigger than 1 and smaller than p
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g,a,p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub,a,p)
}

pub fn modular_exponentiation(base: u64, exponent: u64, modulo: u64) -> u64 {
    // this problem is known as modular exponentiation
    // after looking on the web, simplest solution seems to be using BigUint which already has useful built-in function
    let base = BigUint::from(base);
    let exponent = BigUint::from(exponent);
    let modulo = BigUint::from(modulo);
    base.modpow(&exponent,&modulo).iter_u64_digits().collect::<Vec<u64>>()[0]
}
fn main() {
    println!("Hello, world!");
}
