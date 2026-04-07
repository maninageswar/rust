fn main(){
    println!("the larges number is {}",cal_larges_of_three_numbers(2,5,5));
}

fn cal_larges_of_three_numbers(n1:i32,n2:i32,n3:i32)->i32 {
    let mut largest_number = n3;

    if n1 > largest_number {
        largest_number = n1;
    } else if n2 > largest_number {
        largest_number = n2;
    } 
    largest_number
}