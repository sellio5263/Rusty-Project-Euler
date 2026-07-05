use rusty_project_euler::helper::factors;
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
        }
        if args.len() > 1 {
            let param = args[1].parse::<u64>().unwrap_or(600851475143);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(n: Option<u64>) -> u64 {
    let n = n.unwrap_or(600851475143);
    let factorization = factors::prime_factors(n);
    *factorization.last().unwrap_or(&0)
}
