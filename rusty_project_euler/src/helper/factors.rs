pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut divisor = 2;

    while num > 1 {
        while num.is_multiple_of(divisor) {
            factors.push(divisor);
            num /= divisor;
        }
        divisor += 1;
    }

    factors
}

pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
