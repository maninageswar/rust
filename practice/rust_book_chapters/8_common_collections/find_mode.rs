use std::collections::HashMap;

fn main() {
    let numbers: Vec<i32> = vec![1, 5, 9, 7, 9, 7, 2, 3, 7, 0, 0, 0, 0, 0];
    // let numbers: Vec<i32> = vec![0];
    let mut count_hashmap = HashMap::new();
    let mut count_of_the_key = 0;
    let mut mode: Option<&i32> = None; 
    if numbers.len() > 0 {
        for number in &numbers {
            *count_hashmap.entry(number).or_insert(0) +=1 ;
        }
        for (key, value) in count_hashmap {
            if value > count_of_the_key {
                mode = Some(key);
                count_of_the_key = value
            }
        }
        if let Some(&mode_value) = mode {
            println!("the mode of the vector: {:?} is {}", numbers, mode_value);
        }
    } else {
        println!("the numbers vector is empty hence no mode");
    }
    
}