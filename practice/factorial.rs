use std::io;

fn main(){
    println!("enter a number");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_input:u32 = user_input.trim().parse().expect("Invalid number please enter a number");

    println!("the factorial of {} is {}",user_input,factorial(user_input));

}

fn factorial(number: u32)-> u64 {
    if number == 0 {
        return 1;
    }
    return number as u64 * factorial(number-1);
}