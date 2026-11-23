use rusty_project_euler::helper::calendar_day::CalendarDay;
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
    let mut count = 0;

    let mut date = CalendarDay::new(1, 1, 1, 1900); // January 1, 1900 is a Monday

    while date.year <= 2000 {
        if date.year >= 1901 && date.weekday == 0 && date.day == 1 {
            count += 1;
        }
        date = date.next();
    }

    count
}
