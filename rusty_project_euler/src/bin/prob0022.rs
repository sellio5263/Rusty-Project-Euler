use rusty_project_euler::helper::words::alphabetical_value;
use std::env;
use std::time::Instant;

const INPUT: &str = include_str!("../data/0022_input.txt");

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
            let _param = args[1].parse::<u32>().unwrap_or(0);
            let result = problem(Some(_param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(_param: Option<u32>) -> u32 {
    let trimmed = INPUT
        .chars()
        .filter(|&c| c != '"' && c != '\n')
        .collect::<String>();
    let mut names = trimmed.split(',').collect::<Vec<&str>>();
    names.sort();

    let mut total_score = 0;
    for (i, name) in names.iter().enumerate() {
        let value = alphabetical_value(name);
        let score = value * (i as u32 + 1);
        total_score += score;
    }
    total_score
}
