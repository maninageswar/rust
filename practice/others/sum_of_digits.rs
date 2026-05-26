use std::io;

fn get_the_user_input() -> String {
    println!("enter the number:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: String = user_input.trim().parse().expect("Invalid number");
    user_input
}

fn main() {
    let user_input: String = get_the_user_input();
    let mut sum: u32 = 0;
    for letter in user_input.chars() {
        // match letter.to_digit(10) {
        //     Some(digit) => {sum += digit},
        //     None => {}
        // }
        // another way to write tha above match if you want to handle only one variant of the Option enum which is return type of to_digit method
        if let Some(digit) = letter.to_digit(10) {
            sum += digit;
        }
    }
    println!("the sum of digits of the number {} is {}",user_input, sum);
}