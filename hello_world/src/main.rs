use rand::Rng;

fn main() {
    let x: u32 = 5;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Hello, world!");
}
