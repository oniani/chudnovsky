// Documentation available at https://docs.rs/rug/1.11.0/rug/index.html
mod lib;
use std::env;

// Default precision
const PRECISION: u32 = 100;

// Number of iterations
const ITERATIONS: u32 = 100;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        print!("Approximation: {:?}\n", lib::pi(PRECISION, ITERATIONS));
        print!("Precision:     {:?}\n", PRECISION);
        print!("Iterations:    {:?}\n", ITERATIONS);
    } else if args.len() == 2 {
        let precision = args[1].parse::<u32>().unwrap();
        if precision <= 0 {
            print!("Precision should be a positive integer\n");
        } else {
            print!("Approximation: {:?}\n", lib::pi(precision, ITERATIONS));
            print!("Precision:     {:?}\n", precision);
            print!("Iterations:    {:?}\n", ITERATIONS);
        }
    } else if args.len() == 3 {
        let precision  = args[1].parse::<u32>().unwrap();
        let iterations = args[2].parse::<u32>().unwrap();
        if precision <= 0 {
            print!("Precision should be a positive integer\n");
        } else {
            print!("Approximation: {:?}\n", lib::pi(precision, ITERATIONS));
            print!("Precision:     {:?}\n", precision);
            print!("Iterations:    {:?}\n", iterations);
        }
    } else {
        print!("Invalid argument(s)");
    }
}
