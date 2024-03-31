use std::io;

fn main() {
    let mut input_str : String;

    loop {
        println!("Enter any temperature in Celsius:");
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        input_str = input;

        match input_str.trim().parse::<f32> () {
            Ok(_num) => break,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
    }

    let input_flt : f32 = match input_str.trim().parse() {
        Ok(a) => a,
        Err(_) => panic!(),
    };

    let fahr = (input_flt * 1.8) + 32.0;

    println!("{input_flt} °C is: {fahr} °F");

}
