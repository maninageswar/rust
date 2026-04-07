fn main() {
    let s:&str = "madam";
    let t : String = s.chars().rev().collect();
    if s == t {
        println!("the string {} is a palindrome",s);
    } else {
        println!("the string {} is not a palindrome",s);
    }
}