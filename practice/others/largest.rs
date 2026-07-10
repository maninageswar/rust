fn find_largest(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    return largest;
}


fn main() {
    let numbers1: Vec<i32> = vec![1, 200, 500, 6];
    println!("the largest number in vec {:?} is {}", &numbers1, find_largest(&numbers1));

    let numbers2: [i32;5] = [1, 2, 3, 4, 5];
    println!("the largest number in the arr {:?} is {}", &numbers2, find_largest(&numbers2));
}