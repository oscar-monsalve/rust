// Traditional if condtional
// fn main() {
//     let a = 10;

//     if a < 5 {
//         println!("The condition is true");
//     } else {
//         println!("The condition is false")
//     }

// }


// The if conditional is an expression, meaning it can be assigned to a variable. Also, the if condition can be written in a single line
fn main() {
    let condition = false;

    let number = if condition {5} else {10};

    println!("The value of number is {number}");

}
