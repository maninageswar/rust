fn main() {
    let normal_string = String::from("hello");

    let mut reversed_string = String::from("");

    for letter in normal_string.chars().rev() {
        reversed_string = reversed_string + &String::from(letter);
    }
    println!("the reverse of {} is {}",normal_string,reversed_string)
}