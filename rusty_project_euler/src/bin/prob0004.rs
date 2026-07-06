use std::env;
use std::time::Instant;

use rusty_project_euler::helper::palindrome;

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
            let param = args[1].parse::<u32>().unwrap_or(3);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(digits: Option<u32>) -> u32 {
    let digits = digits.unwrap_or(3);
    let mut largest_palindrome = 0;
    let mut a = 10_u32.pow(digits) - 1;
    while a >= 100 {
        let mut b = 10_u32.pow(digits) - 1;
        while b >= a {
            if a * b <= largest_palindrome {
                break;
            }
            if palindrome::is_palindrome(a * b) {
                largest_palindrome = a * b;
            }
            b -= 1;
        }
        a -= 1;
    }
    largest_palindrome
}
