// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a75a073-518c-83ee-b6c4-9036707c905a
mod calculator;

use calculator::{*, multiply::*};
// use calculator::multiply::*;

fn main() {
    let (addition, subtraction) = do_addtion_and_subtract(6,8);
    println!("addition(6,8): {}, subtraction(6,8): {}", addition, subtraction);

    println!("multiplication(9,4): {}, ", multiply(9,4));
}
