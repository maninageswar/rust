fn append_value<'a>(v: &'a mut Vec<&'a i32>, value: &'a i32) {
    v.push(value);
}

fn main() {
    let a: i32 = 1;
    let mut numbers: Vec<&i32> = vec![&a];
    let value_to_be_added: i32 = 5;
    append_value(&mut numbers, &value_to_be_added);
    println!("the number vector is {:?}", numbers);
}