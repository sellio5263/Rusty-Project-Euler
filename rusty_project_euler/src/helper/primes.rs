pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        // 1 and 0 are not prime numbers
        false
    } else if n <= 3 {
        // 2 and 3 are prime numbers
        true
    } else if n.is_multiple_of(2) {
        // Even numbers greater than 2 are not prime
        false
    } else if n <= 8 {
        // Remaining numbers 5 and 7 are prime
        true
    } else if n.is_multiple_of(3) {
        // Multiples of 3 greater than 3 are not prime
        false
    } else {
        let root = n.isqrt(); // Largest possible lower factor of n is its integer square root
        let mut k = 1;
        while 6 * k - 1 <= root {
            // All primes greater than 3 can be expressed as 6k ± 1
            if n.is_multiple_of(6 * k - 1) || n.is_multiple_of(6 * k + 1) {
                return false;
            }
            k += 1;
        }
        true // If no factors were found, n is prime
    }
}

pub fn sieve_of_eratosthenes_vector(max: u32) -> Vec<bool> {
    let upper_bound = max as usize;
    let mut sieve_array = vec![true; (max + 1) as usize];

    sieve_array[0] = false;
    sieve_array[1] = false;

    for i in (4..=upper_bound).step_by(2) {
        sieve_array[i] = false;
    }

    let mut i = 3;
    while i * i <= upper_bound + 1 {
        if sieve_array[i] {
            for j in (i * i..=upper_bound).step_by(2 * i) {
                sieve_array[j] = false;
            }
        }
        i += 2;
    }

    sieve_array
}
