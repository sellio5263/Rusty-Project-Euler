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
            let param = args[1].parse::<u32>().unwrap_or(10001);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(index: Option<u32>) -> u32 {
    let index = index.unwrap_or(10001);
    let mut count = 6; // Starting from the 6th prime number (13)
    let mut candidate = 15; // Starting from the next odd number after 13
    while count < index {
        candidate += 2; // Increment by 2 to check only odd numbers
        if primes::is_prime(candidate) {
            count += 1;
        }
    }
    candidate
}
