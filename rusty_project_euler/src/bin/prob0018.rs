use std::cmp::max;
use std::env;
use std::time::Instant;

const INPUT: &str = "75\n\
                     95 64\n\
                     17 47 82\n\
                     18 35 87 10\n\
                     20 04 82 47 65\n\
                     19 01 23 75 03 34\n\
                     88 02 77 73 07 63 67\n\
                     99 65 04 28 06 16 70 92\n\
                     41 41 26 56 83 40 80 70 33\n\
                     41 48 72 33 47 32 37 16 94 29\n\
                     53 71 44 65 25 43 91 52 97 51 14\n\
                     70 11 33 28 77 73 17 78 39 68 17 57\n\
                     91 71 52 38 17 14 91 43 58 50 27 29 48\n\
                     63 66 04 68 89 53 67 30 73 16 69 87 40 31\n\
                     04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

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
    let mut triangle = get_triangle();

    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            triangle[i][j] += max(triangle[i + 1][j], triangle[i + 1][j + 1]);
            // println!("i: {}, j: {}, value: {}", i, j, triangle[i][j]);
        }
    }
    triangle[0][0]
}

fn get_triangle() -> Vec<Vec<u32>> {
    let mut triangle: Vec<Vec<u32>> = Vec::new();
    for line in INPUT.lines() {
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        triangle.push(row);
    }
    triangle
}
