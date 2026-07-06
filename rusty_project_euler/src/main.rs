use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let mut active = true;
    while active {
        let (command, args) = get_user_input();
        match command.as_str() {
            "exit" => {
                active = false;
                println!("Exiting RPE Shell...");
            }
            "help" => {
                print_help_message();
            }
            "list" => {
                list_problems();
            }
            "run" => {
                if let Some(arguments) = args {
                    let problem_number = arguments[0].parse::<u32>().unwrap_or(0);
                    if problem_number == 0 {
                        println!("Error: Invalid problem number.");
                        continue;
                    }
                    run_problem(problem_number, Some(arguments[1..].to_vec()));
                } else {
                    println!("Error: No problem number provided.");
                }
            }
            "time" => {
                if let Some(arguments) = args {
                    let problem_number = arguments[0].parse::<u32>().unwrap_or(0);
                    if problem_number == 0 {
                        println!("Error: Invalid problem number.");
                        continue;
                    }
                    time_problem(problem_number);
                } else {
                    println!("Error: No problem number provided.");
                }
            }
            _ => {
                println!("Unknown command: {}", command);
                print_help_message();
            }
        }
    }
}

fn get_user_input() -> (String, Option<Vec<String>>) {
    print!("RPE Shell> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

    (command, if args.is_empty() { None } else { Some(args) })
}

fn print_help_message() {
    println!("Rusty Project Euler (RPE) Shell");
    println!("Available commands:");
    println!(
        "  run <problem_number> [<args>] - Runs the specified problem with optional arguments"
    );
    println!("  time <problem_number> - Runs the specified problem and measures execution time");
    println!("  list - Lists all available problems");
    println!("  help - Displays this help message");
    println!("  exit - Exits the RPE Shell");
}

fn get_list_problems() -> Vec<u32> {
    let mut problems = Vec::new();
    for i in 1..=9999 {
        let problem_file = format!("src/bin/prob{:04}.rs", i);
        if Path::new(&problem_file).exists() {
            problems.push(i);
        }
    }
    problems
}

fn list_problems() {
    let problems = get_list_problems();
    if problems.is_empty() {
        println!("No problems found.");
    } else {
        println!("Available problems:");
        for problem in problems {
            println!("  Problem {}", problem);
        }
    }
}

fn run_problem(problem_number: u32, args: Option<Vec<String>>) {
    if !get_list_problems().contains(&problem_number) {
        println!("Error: Problem {} not found.", problem_number);
        return;
    }
    let mut command = Command::new("cargo");
    command.args(["run", "--bin", &format!("prob{:04}", problem_number)]);
    if let Some(arguments) = args {
        command.arg("--");
        command.args(arguments);
    }
    // dbg!(&command);
    let output = command
        .output()
        .unwrap_or_else(|_| panic!("Problem {} Failed: ", problem_number));
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("Problem {} Output:\n{}", problem_number, stdout);
}

fn time_problem(problem_number: u32) {
    if !get_list_problems().contains(&problem_number) {
        println!("Error: Problem {} not found.", problem_number);
        return;
    }
    run_problem(problem_number, Some(vec!["--time".to_string()]));
}

/*
CLI Flow:
RPE Shell>

Possible Commands:
    run <problem_number> [<args>] - Runs the specified problem with optional arguments
    time <problem_number> - Runs the specified problem and measures execution time
    list - Lists all available problems
    help - Displays this help message
    exit - Exits the RPE Shell
*/
