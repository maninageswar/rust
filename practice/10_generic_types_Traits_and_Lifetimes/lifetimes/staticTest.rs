fn bad() -> &'static str {
    let s = String::from("hello");
    &s
}

fn main() {
    println!("the string is {}",bad());
}