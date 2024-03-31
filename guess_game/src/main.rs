use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The random number is: {secret_number}");

    // Create a loop to ask the user to enter a number

    loop {
        println!("Enter a number:");

        // Creating the "guess" variable. It has to be mutable (mut)
        let mut guess = String::new();

        // Reading input from user using io
        io::stdin()
        .read_line(&mut guess)
        .expect("Invalid input");

        // Convert the string "guess" to an int type (u32) to be able to compare it with the int random number
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small"),
            Ordering::Greater => println!("{guess} is too big"),
            Ordering::Equal => {
                println!("You guessed the number!");
                break;
            }
        }
    }

}
