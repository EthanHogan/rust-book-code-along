use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
fn main() {
    loop {
        // Starts to chug a bit at around the 150000th fib
        println!("Which Nth Fibonnaci number would you like to know? (Q to quit)");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n = n.trim();

        if n == "Q" {
            break;
        }

        let n: usize = match n.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Must be a valid, positive integer or 'Q' to quit!");
                println!();
                continue;
            }
        };

        println!("The {}th Fibonacci number is {}", n, fibonacci(n));
        println!();

        fn fibonacci(n: usize) -> BigUint {
            if n == 0 {
                return Zero::zero();
            } else if n == 1 {
                return One::one();
            }

            let mut current_nth: usize = 2;
            let mut previous_two_fibs: [BigUint; 2] = [Zero::zero(), One::one()];

            while current_nth < n {
                current_nth += 1;

                let new_fib = &previous_two_fibs[0] + &previous_two_fibs[1];

                previous_two_fibs[0] = previous_two_fibs[1].clone();
                previous_two_fibs[1] = new_fib;
            }

            previous_two_fibs[1].clone()
        }
    }
}
