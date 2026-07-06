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
            let param = args[1].parse::<u32>().unwrap_or(20);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(max_divisor: Option<u32>) -> u32 {
    let max_divisor = max_divisor.unwrap_or(20);
    let mut factors = Vec::<u32>::new();
    for i in 2..=max_divisor {
        let i_factors = factors::prime_factors(i as u64);
        for f in &i_factors {
            let factor_count_f = factors.iter().filter(|&&x| x == *f as u32).count();
            let i_factors_count_f = i_factors.iter().filter(|&&x| x == *f).count();
            if factor_count_f < i_factors_count_f {
                let count_to_add = i_factors_count_f - factor_count_f;
                for _ in 0..count_to_add {
                    factors.push(*f as u32);
                }
            }
        }
    }
    factors.iter().product()
}
