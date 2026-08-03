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
            return None;
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
    name: String,
    equipments: Vec<T>,
}

impl<T: Equipment> RentalShop<T> {
    fn add_equipment(&mut self, equipment:T) {
        self.equipments.push(equipment);
    }

    fn remove_equipment(&mut self, id: u32) {
        self.equipments.retain(|equipment| equipment.get_equipment_id() != id);
    }

    fn rent_equipment(&mut self, id: u32, customer: Customer, rental_duration: Duration) {
        let is_equipment_rented = self.equipments.iter_mut()
            .find(|equipment| equipment.get_equipment_id() == id)
            .unwrap_or_else(|| panic!("rent equipment: we did not find an equipment that has an id: {} in the RentalShop", id))
            .rent_equipment(customer, rental_duration);
        
        if let Some(equipment_rented_status) = is_equipment_rented {
            if equipment_rented_status == true {
                println!("thank you so much for renting the equipment with id: {}", id);
            } else {
                // None
            }
        } else {
            panic!("sorry the equipment with id: {} has already been rented", id);
            // None
        }
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

impl<'a, 'b, T: Equipment>  RentalHistory<'a, 'b, T> {
    fn new(name: String) -> Self {
        Self {
            name,
            rental_history: Vec::<RentalRecord<'a, 'b, T>>::new(),
        }
    }
}

