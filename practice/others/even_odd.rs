use std::io;

fn main() {
    println!("enter a number");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_input:i32 = user_input.trim().parse().expect("Invalid number please enter a number");

    if user_input % 2 == 0 {
        println!("your input is a even number")
    } else {
        println!("you input is a odd number")
    }
}