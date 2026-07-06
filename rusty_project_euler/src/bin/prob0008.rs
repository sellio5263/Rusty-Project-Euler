use std::env;
use std::time::Instant;

const TEST_STRING: &str = "731671765313306249192251196744265747423553491949349698352031277450632623957831801698480186947885184385861560789112\
                            94949545950173795833195285320880551112540698747158523863050715693290963295227443043557668966489504452445231617318\
                            56403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585\
                            93079608667017242712188399879790879227492190169972088809377665727333001053367881220235421809751254540594752243525\
                            84907711670556013604839586446706324415722155397536978179778461740649551492908625693219784686224828397224137565705\
                            60574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465\
                            67481391912316282458617866458359124566529476545682848912883142607690042242190226710556263211111093705442175069416\
                            58960408071984038509624554443629812309878799272442849091888458015616609791913387549920052406368991256071760605886\
                            11646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

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
            let param = args[1].parse::<u32>().unwrap_or(13);
            let result = problem(Some(param));
            println!("Result: {}", result)
        }
    } else {
        let result = problem(None);
        println!("Result: {}", result);
    }
}

fn problem(substr_len: Option<u32>) -> u64 {
    let substr_len = substr_len.unwrap_or(13);
    let mut start_idx = 0;
    let mut end_idx = start_idx + substr_len as usize;
    let mut max_product = 0;
    while end_idx < TEST_STRING.len() {
        let substr = &TEST_STRING[start_idx..end_idx];
        let product = substr
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .product::<u64>();
        if product > max_product {
            max_product = product;
        }
        start_idx += 1;
        end_idx += 1;
    }
    max_product
}
