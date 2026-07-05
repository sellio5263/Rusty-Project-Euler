use std::env;
use std::time::Instant;

fn main() {
    // Flag Possible: --time|-t: Measure execution time
    let args = env::args().collect::<Vec<String>>();
    dbg!(&args);
    if args.len() > 1 {
        if args.contains(&"--time".into()) || args.contains(&"-t".into()) {
            let start = Instant::now();
            let result = problem(None);
            let duration = start.elapsed();
            println!("Result: {}", result);
            println!("Time elapsed: {:?}", duration);
        }
        if args.len() > 1 {
            let upper_bound = args[1].parse::<u32>().unwrap_or(1000);
            let result = problem(Some(upper_bound));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(upper_bound: Option<u32>) -> u32 {
    let upper_bound = upper_bound.unwrap_or(1000);
    (3..upper_bound).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
