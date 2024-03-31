use std::io;

fn main() {
    println!("Enter any word");

    let mut word = String::new();

    io::stdin()
    .read_line(&mut word)
    .expect("Invalid input");

    let word_index = word[0];
}
