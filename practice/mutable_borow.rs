fn main() {
    let mut x: u32 = 1;
    println!("the number before increment is {}",x);
    increase_x(&mut x);
    println!("the number after increment is {}",x);

    let mut word: String = String::from("hello");
    append_world(&mut word);
    println!("the entire word is {}",word);
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn increase_x(x: &mut u32) {
    *x = *x + 1;
}