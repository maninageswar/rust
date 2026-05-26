fn swap<T, U>(arg1: T, arg2: U) -> (U, T) {
    (arg2, arg1)
}

fn main() {
    let (a, b) = swap(3.3, "hello");
    println!("the a, b are {}, {}", a, b);
}