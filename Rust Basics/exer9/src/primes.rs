pub fn factors_iterator(n: u64) -> impl Iterator<Item = u64> {
    return (2..=n / 2).filter(move |x| n % x == 0);
}

pub fn factors(n: u64) -> Vec<u64> {
    return factors_iterator(n).collect();
}

pub fn is_prime(n: u64) -> bool {
    return factors_iterator(n).next() == None;
}
