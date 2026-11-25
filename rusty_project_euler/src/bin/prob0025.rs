use rusty_project_euler::helper::mega_int::MegaUInt;
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
            let param = args[1].parse::<u32>().unwrap_or(1000);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(param: Option<u32>) -> u32 {
    let digit_count = param.unwrap_or(1000) as usize;
    // Implementation for finding the index-th Fibonacci number with the specified digit count
    let mut two_prev = MegaUInt::one();
    let mut prev = MegaUInt::one();
    let mut index = 2; // Starting from the 3rd Fibonacci number
    let mut current = MegaUInt::zero();
    while current.digits().len() < digit_count {
        current = two_prev.add(&prev);
        two_prev = prev;
        prev = current.clone();
        index += 1;
    }
    index
}
