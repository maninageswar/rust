fn remove_duplicate_elements_from_list<T: std::cmp::PartialEq>(list: &mut Vec<T>) {
    let mut indexes_to_remove: Vec<usize> = vec![];
    for i in 0..list.len() {
        if i < list.len() - 1 {
            for j in i+1..list.len() {
                if list[i] == list[j] {
                    if !indexes_to_remove.contains(&j) {
                        indexes_to_remove.push(j);
                    }
                }
            }
        }
    }

    indexes_to_remove.reverse();
    // println!("indexes_to_remove : {:?}", indexes_to_remove);
    for ind in indexes_to_remove {
        list.remove(ind);
    }

}

fn main() {
    let mut numbers_list: Vec<i32> = vec![1, 2, 3, 3, 5, 6, 6];
    remove_duplicate_elements_from_list(&mut numbers_list);
    println!("the numbers_list after removing the duplicates is {:?}", numbers_list);
    println!();

    let mut char_list: Vec<char> = vec!['a', 'a', 'a', 'b', 'c'];
    remove_duplicate_elements_from_list(&mut char_list);
    println!("the char_list after removing the duplicates is {:?}", char_list);
    println!();

    let mut str_list = vec!["apple", "banana", "apple", "cherry", "banana"];
    remove_duplicate_elements_from_list(&mut str_list);
    println!("the str_list after removing the duplicates is {:?}", str_list);
    println!();

    let mut string_list = vec![
        String::from("hello"), 
        String::from("world"), 
        String::from("hello"), 
        String::from("rust")
    ];
    remove_duplicate_elements_from_list(&mut string_list);
    println!("the string_list after removing the duplicates is {:?}", string_list);
    println!();
}