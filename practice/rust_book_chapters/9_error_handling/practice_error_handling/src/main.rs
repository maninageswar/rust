mod guessing_number;
use guessing_number::{Guess, GuesString};

fn main() {
    let guess: Guess = Guess::new(300);
    let s = guess.value();
    println!("the s is {}",s);
    println!("the guessed value is {}",guess.value());

    let st = String::from("hello");
    let guess2: GuesString = GuesString::new(st);
    let s2 = guess2.value();
    println!("the s is {}",s2);
    println!("the guessed value is {}",guess2.value());
}
