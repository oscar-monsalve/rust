use std::io;

fn main() {
    // Create inmutables variables with "let"

    // let x = 5;
    // let y = 5;
    // let sum = x+y;
    // println!("The sum of x and y is: {sum}");

    // __________________________________________________

    // Shadowing a variable. The variable to be shadowed must be muted (mut)
    // let mut x = 5;
    // println!("x is: {x}");
    // x = 6;
    // println!("x is: {x}");

    // __________________________________________________

    // Creating constants: 1. The constant name must be upper case. 2. The variable type must be specified
    // const CONVERSION_H_TO_S : u32 = 60 * 60 * 3;
    // println!("3 hours in seconds is {CONVERSION_H_TO_S} seconds");

    // __________________________________________________

    // Shadowing and scopes.
    // let x = 5;
    // let x = x+1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}")
    // }

    // println!("The value of x out of the scope is: {x}");

    // __________________________________________________

    // Ex.: shadowing a variable without mut
    // let spaces = "    ";
    // let spaces = spaces.len();

    // println!("The number of spaces the user wants is: {spaces}");

    // __________________________________________________

    // // Scalar types
    //     // Unsigned integer type cannot be negative. Ranges from 8 bit to 128 bit.
    //     let a : u8 = 1;
    //     let b : u16 = 2;
    //     let c : u32 = 3;
    //     let d : u64 = 4;
    //     let e : u128 = 5;
    //     let f : usize = 6;
    //     println!("{a}, {b}, {c}, {d}, {e}, {f}");

    //     // signed integer type can be negative or positive. Ranges from 8 bit to 128 bit.
    //     let aa : i8 = -1;
    //     let bb : i16 = 2;
    //     let cc : i32 = -3;
    //     let dd : i64 = 4;
    //     let ee : i128 = -5;
    //     let ff : isize = 6;
    //     println!("{aa}, {bb}, {cc}, {dd}, {ee}, {ff}");

    // __________________________________________________

//    let x : f64 = -5.0/3.0;
//    println!("{x}");

    // let f = true;
    // let a : bool = false;

    // Char literal are used with sigle quotes ''
    // let c: char = 'ds';
    // let f : &str = "ds";

// __________________________________________________

    // Tuples. Use "{variable:?}" to print tuples

    // let tup = (500, 50.0, true);

        // // Accessing a tuple
        // let (x,y,z) = tup;
        // println!("The value of y is: {y}, and the value of z is: {z}");
        // // or
        // let acc = tup.0;
        // println!("{acc}")

    // Arrays

    // let a : [i32; 4] = [0, 1, 2, 3];
    // let b = [3;5];
    // println!("{a:?}");
    // println!("{b:?}");

        // Accessing an array

        // let first_in_a = a[0];
        // let second_in_a = a[1];

        // println!("First in a is: {first_in_a}");
        // println!("Second in a is: {second_in_a}");

// __________________________________________________

    // Exercise. Write a program that asks the user to enter an index to show the corresponding value position in a array.

    let a : [i32; 5] = [1, 2, 3, 4, 5];

    println!("Enter an array index:");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Invalid input");

    let input : usize = input
    .trim()
    .parse()
    .expect("The input was not a number");

    let a_pos = a[input];

    println!("The index {input} corresponds to a value in the array of: {a_pos}");
}
