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

pub fn num_factors(n: u32) -> u32 {
    let mut factor_count = 0;
    for i in 1..=n.isqrt() {
        if n.is_multiple_of(i) {
            factor_count += 1;
        }
    }
    factor_count *= 2;
    if (n as f32).sqrt() == n.isqrt() as f32 {
        // dbg!(n);
        // dbg!((n as f32).sqrt());
        // dbg!(n.isqrt() as f32);
        factor_count -= 1;
    }
    factor_count
}

pub fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum = 0;

    for i in 1..=n.isqrt() {
        if n.is_multiple_of(i) {
            sum += i;
            if i != 1 && i != n / i {
                sum += n / i;
            }
        }
    }

    sum
}

pub fn is_abundant(n: u32) -> bool {
    sum_proper_divisors(n) > n
}
