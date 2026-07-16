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
            let param = args[1].parse::<u32>().unwrap_or(20);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(grid_size: Option<u32>) -> u64 {
    let grid_size = grid_size.unwrap_or(20);
    let mut result: f64 = 1.0;
    for i in 1..=grid_size {
        result *= (grid_size as f64 + i as f64) / i as f64;
        // dbg!(i);
        // dbg!(result);
    }
    result.round() as u64
}
