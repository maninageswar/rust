fn main() {
    let mut a: u32 = 10;
    let mut b: u32 = 20;
    swap(&mut a, &mut b);
    println!("{}",a);
    println!("{}",b);
}

fn swap(a: &mut u32, b: &mut u32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}