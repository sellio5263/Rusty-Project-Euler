use rusty_project_euler::helper::primes;
use std::env;
use std::time::Instant;

fn main() {
    // Flag Possible: --time|-t: Measure execution time
    let args = env::args().collect::<Vec<String>>();
    // dbg!(&args);
    if args.len() > 1 {
        if args.contains(&"--time".into()) || args.contains(&"-t".into()) {
            let start = Instant::now();
            let result = problem(None);
            let duration = start.elapsed();
            println!("Result: {}", result);
            println!("Time elapsed: {:?}", duration);
        } else {
            let param = args[1].parse::<u32>().unwrap_or(2000000);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(upper_bound: Option<u32>) -> u64 {
    let upper_bound = upper_bound.unwrap_or(2000000);
    let sieve = primes::sieve_of_eratosthenes_vector(upper_bound);
    let mut sum = 0_u64;
    for i in 2..=upper_bound {
        if sieve[i as usize] {
            sum += i as u64;
        }
    }
    sum
}
