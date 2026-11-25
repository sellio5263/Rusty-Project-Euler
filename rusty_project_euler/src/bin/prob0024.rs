use factorial::Factorial;
// use itertools::Itertools;
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

fn problem(param: Option<u32>) -> u64 {
    let mut index = param.unwrap_or(1000000) - 1; // Convert to 0-based index
    // Implementation for finding the index-th lexicographic permutation
    let mut permuted_digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result: u64 = 0;

    while !permuted_digits.is_empty() {
        let step_size = (permuted_digits.len() as u32 - 1).factorial();
        let d_index = index / step_size;
        let next_digit = permuted_digits.remove(d_index as usize);
        // Update index for the next digit
        index %= step_size;
        result = result * 10 + next_digit as u64;
    }
    result
}

// fn _problem2(param: Option<u32>) -> u64 {
//     let index = param.unwrap_or(1000000);
//     // Implementation for finding the index-th lexicographic permutation

//     let perms = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
//         .into_iter()
//         .permutations(10);

//     for (i, p) in perms.enumerate() {
//         if i == index as usize - 1 {
//             let result = p.iter().map(|&d| d.to_string()).collect::<String>();
//             return result.parse::<u64>().unwrap();
//         }
//     }
//     panic!("Index out of bounds");
// }
