pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 { return false; }
    }
    true
}
