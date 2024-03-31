use std::io;
fn main() {
    let mut input_str : String;

    loop {
        println!("Enter a number to generate as many Fibonacci numbers: ");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        input_str = input;

        match input_str.trim().parse::<i32> () {
            Ok(_num) => break,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

    }

    let input_int : i32 = match input_str.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!()
    };

    let mut init: i32 = 1;
    let mut prev: i32 = 0;

    println!("Generating {input_int} Fibonacci numbers:");

    for _i in 1..=input_int {
        let next = init + prev;
        prev = init;
        init = next;
        println!("{next}");
    }

}
