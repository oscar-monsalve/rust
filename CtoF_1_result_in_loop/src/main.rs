// Convert temperatures between Celsius and Fahrenheit

use std::io;

fn main() {
    loop {
        println!("Enter a temperature in Celsius:");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

        let input : f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        if input >= -273.15 {
            println!("The entered temperature in Celsius is: {input}");
            let fahr : f32 = (input * 1.8) + 32.0;
            println!("{input} degree Celsius is {fahr} degree Fahrenheit");
            break;
        }

    }


}
