// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a676f87-8cac-83e8-aeef-e3aaa41bae98

use mini_equipment_rental_system::{models::*};

fn main() {
    println!("Hello, world!");
    let laptop1: Laptop = Laptop {
        id: 1,
        brand: String::from("apple"),
        processor: String::from("apple-silicon-M3"),
        ram: String::from("16GB"),
        status: Status::Available,
    };
    println!("the laptop is {:#?}", laptop1);
}
