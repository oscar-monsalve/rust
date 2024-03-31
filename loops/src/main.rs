// Repeating code with loop. Loops can return a value to use after the loop breaks

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break
            counter * 2
        }
    };

    println!("The result is {result}");
}
