// Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1.
// Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.
// Given a number n, return the number of steps required to reach 1.

use std::io;

fn main() {
    let mut input_str: String;

    loop {
        println!("Enter any positive integer:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input_str = input;

        match input_str.trim().parse::<i32>() {
            Ok(_num) => break,
            Err(_) => {
                println!("Invalid input. \n");
                continue;
            }
        };
    }

    let mut input_int: i32 = match input_str.trim().parse() {
        Ok(a) => a,
        Err(_) => panic!(),
    };

    let mut count = 0;

    loop {
        if input_int == 1 {
            println!("The program has ended with a value of {input_int}");
            break;
        } else if input_int % 2 == 0 {
            input_int = input_int / 2
        } else if input_int % 2 != 0 {
            input_int = 3 * input_int + 1
        }

        count += 1
    }

    println!("It took {count} cycles to reach 1.");

}
