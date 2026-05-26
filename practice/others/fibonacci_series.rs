use std::io;

fn get_the_user_input() -> u32 {

    println!("enter n:");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_input: u32 = user_input
        .trim()
        .parse()
        .expect("Invalid number");

    user_input
}

fn main() {
    for i in 0..get_the_user_input() {
        print!("{} ", f(i))
    }
    println!();
}

fn f(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return f(n-1) + f(n-2);
    }
}