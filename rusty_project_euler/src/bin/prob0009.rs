use rusty_project_euler::helper::factors;
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

fn problem(target_sum: Option<u32>) -> u32 {
    let target_sum = target_sum.unwrap_or(1000);
    let half_sum = target_sum / 2;
    let max_m = half_sum.isqrt();
    for m in 2..=max_m {
        if half_sum.is_multiple_of(m) {
            let mut mth_half_sum = half_sum / m;
            while mth_half_sum.is_multiple_of(2) {
                mth_half_sum /= 2;
            }
            let mut k;
            if m % 2 == 1 {
                k = m + 2;
            } else {
                k = m + 1;
            }
            while k < 2 * m && k <= mth_half_sum {
                if mth_half_sum.is_multiple_of(k) && factors::gcd(k, m) == 1 {
                    let d = half_sum / (k * m);
                    let n = k - m;
                    let a = d * (m * m - n * n);
                    let b = 2 * d * m * n;
                    let c = d * (m * m + n * n);
                    return a * b * c;
                }
                k += 2;
            }
        }
    }
    0
}
