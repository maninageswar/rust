// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a676f87-8cac-83e8-aeef-e3aaa41bae98

use mini_equipment_rental_system::{*, models::*};
// use mini_equipment_rental_system::*;

fn main() {
    println!("Hello, world!");
    // let laptop1: Laptop = Laptop {
    //     id: 1,
    //     brand: String::from("apple"),
    //     processor: String::from("apple-silicon-M3"),
    //     ram: String::from("16GB"),
    //     status: Status::Available,
    // };
    let mut laptop1: Laptop = Laptop::new(1, String::from("apple"), String::from("apple-silicon-M3"), String::from("16GB"), Status::Available);
    let mut laptop2: Laptop = Laptop::new(1, String::from("apple"), String::from("apple-silicon-M4"), String::from("16GB"), Status::Available);
    let mut laptop3: Laptop = Laptop::new(1, String::from("apple"), String::from("apple-silicon-M5"), String::from("16GB"), Status::Available);

    let mut laptop_rental_shop1: RentalShop<Laptop> = RentalShop {
        name: String::from("laptop_rental_shop1"),
        equipments: Vec::<Laptop>::new(),
    };

    let customer1: Customer = Customer::new(1, String::from("sai shankar"), 1234567890);

    let mut laptop_rental_history1: RentalHistory<Laptop> = RentalHistory::new(String::from("laptop_rental_history1"));

    laptop_rental_shop1.add_equipment(laptop1);
    laptop_rental_shop1.add_equipment(laptop2);
    laptop_rental_shop1.add_equipment(laptop3);

    println!("\nbefore renting laptop");
    laptop_rental_shop1.list_all_equipments();

    match laptop_rental_shop1.rent_equipment(1, &customer1, Duration::from_secs((3 * 3600) + (2 * 60) + 7)) {
        Some(rental_record) => laptop_rental_history1.add_rental_record(rental_record),
        None => ()
    }
    println!("\nafter renting laptop");
    laptop_rental_shop1.list_all_rented_equipments();

    println!("\nlaptop_rental_history1 is ");
    println!("\n{:#?}", laptop_rental_history1);
}
