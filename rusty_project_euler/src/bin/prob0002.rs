use rusty_project_euler::helper::fibonacci;
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
            let param = args[1].parse::<u32>().unwrap_or(4000000);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(upper_bound: Option<u32>) -> u32 {
    let upper_bound = upper_bound.unwrap_or(4000000);
    let mut a: u32 = 1;
    let mut b: u32 = 2;
    let mut sum: u32 = 2;
    let mut next: u32 = 0;

    while next < upper_bound {
        next = fibonacci::next_fibonacci(a, b);
        if next.is_multiple_of(2) {
            sum += next;
        }
        a = b;
        b = next;
    }

    sum
}
