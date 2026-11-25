use rusty_project_euler::helper::factors::sum_proper_divisors;
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
            let param = args[1].parse::<u32>().unwrap_or(10000);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(param: Option<u32>) -> u32 {
    let upper_limit = param.unwrap_or(10000);
    let mut amicable_numbers: Vec<u32> = Vec::new();
    for a in 1..upper_limit {
        let b = sum_proper_divisors(a);
        if a != b && sum_proper_divisors(b) == a {
            amicable_numbers.push(a);
        }
    }
    amicable_numbers.iter().sum()
}
