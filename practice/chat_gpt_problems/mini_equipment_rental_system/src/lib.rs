pub mod models;
use models::*;
// the below line is not needed as it is re-exported in models module and since you are using every thing that is avaliable in models using "use models::*;" so Duration is also included
// use std::time::Duration;

pub trait Equipment {
    fn get_equipment_id(&self) -> u32;

    fn is_equipment_avaliable(&self) -> bool;

    fn rent_equipment(&mut self, customer: Customer, rental_duration: Duration) -> Option<bool>;

    fn print_equipment(&self);
}

impl Equipment for Camera {
    fn get_equipment_id(&self) -> u32 {
        self.id
    }

    fn is_equipment_avaliable(&self) -> bool {
        if self.status == Status::Available {
            return true;
        } else {
            return false;
        }
    }

    fn rent_equipment(&mut self, customer: Customer, rental_duration: Duration) -> Option<bool> {
        if self.is_equipment_avaliable() == true {
            self.status = Status::Rented {
                customer,
                rental_duration
            };
            return Some(true);
        } else {
            println!("sorry, the Equipment is not avaliable to be rented");
            return Some(false);
        }
    }

    fn print_equipment(&self) {
        println!("the Equipment is {:#?}", self)
    }
}

impl Equipment for Laptop {
    fn get_equipment_id(&self) -> u32 {
        self.id
    }

    fn is_equipment_avaliable(&self) -> bool {
        if self.status == Status::Available {
            return true;
        } else {
            return false;
        }
    }

    fn rent_equipment(&mut self, customer: Customer, rental_duration: Duration) -> Option<bool> {
        if self.is_equipment_avaliable() == true {
            self.status = Status::Rented {
                customer,
                rental_duration
            };
            return Some(true);
        } else {
            println!("sorry, the Equipment is not avaliable to be rented");
            return None;
        }
    }

    fn print_equipment(&self) {
        println!("the Equipment is {:#?}", self)
    }
}

impl Equipment for Projector {
    fn get_equipment_id(&self) -> u32 {
        self.id
    }

    fn is_equipment_avaliable(&self) -> bool {
        if self.status == Status::Available {
            return true;
        } else {
            return false;
        }
    }

    fn rent_equipment(&mut self, customer: Customer, rental_duration: Duration) -> Option<bool> {
        if self.is_equipment_avaliable() == true {
            self.status = Status::Rented {
                customer,
                rental_duration
            };
            return Some(true);
        } else {
            println!("sorry, the Equipment is not avaliable to be rented");
            return None;
        }
    }

    fn print_equipment(&self) {
        println!("the Equipment is {:#?}", self)
    }
}

impl Equipment for Microphone {
    fn get_equipment_id(&self) -> u32 {
        self.id
    }

    fn is_equipment_avaliable(&self) -> bool {
        if self.status == Status::Available {
            return true;
        } else {
            return false;
        }
    }

    fn rent_equipment(&mut self, customer: Customer, rental_duration: Duration) -> Option<bool> {
        if self.is_equipment_avaliable() == true {
            self.status = Status::Rented {
                customer,
                rental_duration
            };
            return Some(true);
        } else {
            println!("sorry, the Equipment is not avaliable to be rented");
            return None;
        }
    }

    fn print_equipment(&self) {
        println!("the Equipment is {:#?}", self)
    }
}

pub struct RentalShop<T: Equipment> {
    pub name: String,
    pub equipments: Vec<T>,
}

impl<T: Equipment> RentalShop<T> {
    pub fn add_equipment(&mut self, equipment:T) {
        self.equipments.push(equipment);
    }

    pub fn remove_equipment(&mut self, id: u32) {
        self.equipments.retain(|equipment| equipment.get_equipment_id() != id);
    }

    pub fn rent_equipment<'a, 'b>(&'a mut self, id: u32, customer: &'b Customer, rental_duration: Duration) -> Option<RentalRecord<'a, 'b, T>> {
        let equipment_to_be_rented: &mut T = self.equipments.iter_mut()
            .find(|equipment| equipment.get_equipment_id() == id)
            .unwrap_or_else(|| panic!("rent equipment: we did not find an equipment that has an id: {} in the RentalShop", id));
        // instead of clonning the customer try to change the code so that you can add the reference
        let is_equipment_rented = equipment_to_be_rented.rent_equipment(customer.clone(), rental_duration);
        // drop(equipment_to_be_rented);
        if let Some(equipment_rented_status) = is_equipment_rented {
            if equipment_rented_status == true {
                println!("thank you so much for renting the equipment with id: {}", id);
                let rental_record: RentalRecord<'a, 'b, T> = RentalRecord::new(equipment_to_be_rented, customer, String::from("04-08-26"));
                return Some(rental_record);
            } else {
                println!("sorry the equipment with id: {} has already been rented", id);
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn search_equipment(&self, id: u32) -> Option<&T> {
        self.equipments.iter().find(|equipment| equipment.get_equipment_id() == id)
    }

    pub fn list_all_equipments(&self) {
        self.equipments.iter().for_each(|item| item.print_equipment());
    }

    pub fn list_all_rented_equipments(&self) {
        self.equipments.iter().for_each(|item| {
            if item.is_equipment_avaliable() == false {
                item.print_equipment()
            }
        });
    }

    pub fn list_all_avaliable_equipments(&self) {
        self.equipments.iter().for_each(|item| {
            if item.is_equipment_avaliable() == true {
                item.print_equipment()
            }
        });
    }
}

#[derive(Debug, PartialEq)]
pub struct RentalRecord<'a, 'b, T: Equipment> {
    pub rented_equipment: &'a T,
    pub customer: &'b Customer,
    pub rental_date: String,
}

impl<'a, 'b, T: Equipment> RentalRecord<'a, 'b, T> {
    fn new(rented_equipment: &'a T, customer: &'b Customer, rental_date: String) -> Self {
        Self {
            rented_equipment,
            customer,
            rental_date,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RentalHistory<'a, 'b, T: Equipment> {
    name: String,
    rental_history: Vec<RentalRecord<'a, 'b, T>>
}

impl<'a, 'b, T: Equipment + std::cmp::PartialEq>  RentalHistory<'a, 'b, T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rental_history: Vec::<RentalRecord<'a, 'b, T>>::new(),
        }
    }

    pub fn add_rental_record(&mut self, rental_record: RentalRecord<'a, 'b, T>) {
        self.rental_history.push(rental_record);
    }

    pub fn remove_rental_record(&mut self, rental_record: RentalRecord<'a, 'b, T>) {
        self.rental_history.retain(|record| record != &rental_record);
    }
}

