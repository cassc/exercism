use primes::{PrimeSet, Sieve};

pub fn nth(n: u32) -> u32 {
    let mut pset = Sieve::new();
    pset.iter().enumerate().skip(n as usize).nth(0).unwrap().1 as u32
}
