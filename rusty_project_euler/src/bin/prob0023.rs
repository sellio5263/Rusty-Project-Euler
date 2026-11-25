use rusty_project_euler::helper::factors::is_abundant;
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
            let param = args[1].parse::<u32>().unwrap_or(28123);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(param: Option<u32>) -> u32 {
    let upper_limit = param.unwrap_or(28123);
    let abundance_lookup = build_abundance_lookup_array(upper_limit);
    let mut sum = 0;

    for a in 1..=upper_limit {
        if !is_sum_of_abundants(a, &abundance_lookup) {
            sum += a;
        }
    }
    sum
}

fn is_sum_of_abundants(n: u32, abundance_lookup: &[bool]) -> bool {
    for a in 12..=n {
        if abundance_lookup[a as usize] && abundance_lookup[(n - a) as usize] {
            return true;
        }
    }
    false
}

fn build_abundance_lookup_array(upper_limit: u32) -> Vec<bool> {
    let mut abundance_lookup = vec![false; (upper_limit + 1) as usize];
    for i in 1..=upper_limit {
        if is_abundant(i) {
            abundance_lookup[i as usize] = true;
        }
    }
    abundance_lookup
}
