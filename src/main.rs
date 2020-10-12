mod lib;
use std::env;

// Default precision
const PRECISION: u32 = 100;

// Default number of iterations
const ITERATIONS: u32 = 100;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        print!("Approximation: {:?}\n", lib::pi(PRECISION, ITERATIONS));
        print!("Precision:     {:?}\n", PRECISION);
        print!("Iterations:    {:?}\n", ITERATIONS);
    } else if args.len() == 2 {
        let precision = args[1]
            .parse::<i32>()
            .expect("Precision should be a positive integer");
        if precision <= 0 {
            print!("Precision should be a positive integer\n");
        } else {
            print!(
                "Approximation: {:?}\n",
                lib::pi(precision as u32, ITERATIONS)
            );
            print!("Precision:     {:?}\n", precision);
            print!("Iterations:    {:?}\n", ITERATIONS);
        }
    } else if args.len() == 3 {
        let precision = args[1]
            .parse::<i32>()
            .expect("Precision should be a positive integer");
        let iterations = args[2]
            .parse::<i32>()
            .expect("Number of iterations should be a positive integer");
        if precision <= 0 {
            print!("Precision should be a positive integer\n");
        } else if iterations <= 0 {
            print!("Number of iterations should be a positive integer\n");
        } else {
            print!(
                "Approximation: {:?}\n",
                lib::pi(precision as u32, iterations as u32)
            );
            print!("Precision:     {:?}\n", precision);
            print!("Iterations:    {:?}\n", iterations);
        }
    } else {
        print!("Invalid argument\n");
    }
}
