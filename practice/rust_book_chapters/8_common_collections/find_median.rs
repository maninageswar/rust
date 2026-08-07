fn main() {
    // let mut numbers: Vec<i32> = vec![];

    // even case
    // let mut numbers: Vec<i32> = vec![1, 0, 7, 2, 9, 6];

    // odd case
    // let mut numbers: Vec<i32> = vec![1, 0, 7, 2, 9, 6, 4];

    // single value case
    // let mut numbers: Vec<i32> = vec![1];

    // two values case
    let mut numbers: Vec<i32> = vec![3, 1];

    print!("the median of the vector: {:?} ", numbers);
    numbers.sort();
    let numbers_length: usize = numbers.len();
    if numbers_length % 2 == 0 {
        let n1 = numbers_length/2;
        let n2 = n1 + 1;
        let median = (&numbers[n1-1] + &numbers[n2-1])/2;
        print!("is {}", median);
    } else {
        let median_index = (numbers_length + 1)/2;
        // 1.
        print!("is {}", &numbers[median_index-1]);

        // 2. the below approach is wrong
        // print!("is {}", numbers.get(median_index-1).unwrap_or("the Vector is empty so no median"));

        // 4.
        // if let Some(&median) = numbers.get(median_index-1) {
        //     print!("is {}", median);
        // } else {
        //     println!("the Vector is empty so no median");
        // }

        // 4.
        // match numbers.get(median_index-1) {
        //  Some(&median) => { print!("is {}", median); },
        //  None => { println!("the Vector is empty so no median"); }
        // }
    }
}