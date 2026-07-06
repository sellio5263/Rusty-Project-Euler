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
            let param = args[1].parse::<u32>().unwrap_or(100);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(upper_bound: Option<u32>) -> u32 {
    let upper_bound = upper_bound.unwrap_or(100);
    let sum_squares = (1..=upper_bound).map(|x| x.pow(2)).sum::<u32>();
    let square_sum = (1..=upper_bound).sum::<u32>().pow(2);
    square_sum - sum_squares
}
