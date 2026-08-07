use crate::models::{Customer, Duration};

pub trait Equipment {
    fn get_equipment_id(&self) -> u32;

    fn is_equipment_avaliable(&self) -> bool;

    fn rent_equipment(&mut self, customer: Customer, rental_duration: Duration) -> Option<bool>;

    fn print_equipment(&self);
}