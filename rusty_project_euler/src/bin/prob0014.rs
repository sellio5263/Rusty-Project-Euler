use rusty_project_euler::helper::sequences;
use std::collections::{self, HashMap};
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
            let param = args[1].parse::<u32>().unwrap_or(1000000);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn _problem_v3(max_start: Option<u32>) -> u32 {
    let max_start = max_start.unwrap_or(1000000) as u64;
    let mut max_length = 0;
    let mut max_seed = 0;
    let mut known_lengths = vec![0u32; max_start as usize];
    known_lengths[1] = 1;
    for i in 1..max_start {
        if i.is_multiple_of(1000) {
            // println!("Done with {} at time: {:?}", i, SystemTime::now());
        }
        let length = sequences::smarter_collatz_len(i, &mut known_lengths);
        if length > max_length {
            max_length = length;
            max_seed = i;
        }
    }
    max_seed as u32
}

fn _problem_v2(max_start: Option<u32>) -> u32 {
    let max_start = max_start.unwrap_or(1000000) as u64;
    let mut max_length = 0;
    let mut max_seed = 0;
    let mut known_lengths: collections::HashMap<u64, u32> = HashMap::new();
    for i in 1..max_start {
        if i.is_multiple_of(1000) {
            // println!("Done with {} at time: {:?}", i, SystemTime::now());
        }
        let length = sequences::smart_collatz_len(i, &mut known_lengths);
        if length > max_length {
            max_length = length;
            max_seed = i;
        }
    }
    max_seed as u32
}

fn _problem_v1(max_start: Option<u32>) -> u32 {
    let max_start = max_start.unwrap_or(1000000) as u64;
    let mut max_length = 0;
    let mut max_seed = 0;
    for i in 1..max_start {
        let length = sequences::Collatz::new_collatz(i).count() as u32;
        if length > max_length {
            max_length = length;
            max_seed = i;
        }
    }
    max_seed as u32
}

fn problem(max_start: Option<u32>) -> u32 {
    // _problem_v1(max_start) // Avg Dbg Mode Run Time ~ 2.75s
    // _problem_v2(max_start) // Avg Dbg Mode Run Time ~ 2.6s
    _problem_v3(max_start) // Avg Dbg Mode Run Time ~80ms
}
