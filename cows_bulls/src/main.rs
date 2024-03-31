use std::io;
use rand::Rng;

fn main() {
    let mut count_tries = 0;

    let rand: i32 = rand::thread_rng().gen_range(0..=9999);
    let vector_rand: Vec<i32> = rand
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).unwrap() as i32)
        .collect();

    println!("The random number is: {rand}");

    loop {
        let mut input: String = String::new();
        let mut input_int: i32 = 0; // Initialized before the loop starts with a default value.
        let mut valid_input: bool = false;
        let mut vector_input: Vec<i32> = Vec::new();

        loop {
        println!("Enter a 4-digit number: ");

            input.clear();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let input_trimmed = input.trim();

            if input_trimmed == "done" {
                println!("The program has finished.");
                break;
            }

            if input_trimmed.chars().all(|i| i.is_digit(10)) {
                if input_trimmed.len() == 4 {
                    vector_input = input_trimmed // Iterating throuhg the 4-digit input in pushing into the vector
                        .chars()
                        .map(|i| i.to_digit(10).unwrap() as i32)
                        .collect::<Vec<i32>>();

                    match input_trimmed.parse::<i32>() {
                        Ok(num) => {
                            input_int = num;
                            valid_input = true; // Condition to decide to show or not the input_int variable after "done".
                            break;
                        }
                        Err(_) => {
                            println!("Could not convert to input.");
                            continue;
                        }
                    };
                }
                else {
                    println!("Error. The number is required to be 4 digits long.");
                    continue;
                }
            }
            else {
                println!("Invalid input.");
            }
        }

        if valid_input {
            println!("The guessed number is: {}", input_int);
        }

        let mut count_bulls = 0;
        let mut count_cows = 0;

        for i in 0..4 {
            if vector_input[i] == vector_rand[i] {
                count_cows += 1
            }
            else if vector_input.contains(&vector_rand[i]) {
                count_bulls += 1
            }
        }

        count_tries += 1;

        if input_int != rand {
            println!("cows: {}, bulls: {} \n", count_cows, count_bulls);
        }
        else if input_int == rand {
            println!("You guessed the number! It took {} tries.", count_tries);
            break;
        }
    }

}
