fn does_list_contain_value<T: std::cmp::PartialEq>(list: &[T], value: &T) -> bool {
    for item in list {
        if item == value { return true; }
    }
    false
}

fn main() {
    let numbers: [i32; 5] = [1, 3, 5, 9, 2];
    let floating_numbers: [f64; 3] = [1.0, 9.3, 2.5];
    let vector_of_strings: Vec<String> = vec![String::from("pavan"), String::from("sumathi"), String::from("kumar")];
    println!("does {:?} contains {} : {}", numbers, 3, does_list_contain_value(&numbers, &3));
    println!("does {:?} contains {} : {}", floating_numbers, 0.0, does_list_contain_value(&floating_numbers, &0.0));
    println!("does {:?} contains {} : {}", vector_of_strings, String::from("pavan"), does_list_contain_value(&vector_of_strings, &String::from("pavan")));
}