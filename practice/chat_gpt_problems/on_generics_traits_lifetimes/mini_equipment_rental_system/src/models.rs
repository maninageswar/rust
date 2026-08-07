// remove the pub keyword in the below line and try to run it in lib.rs file wherever you used Duration
pub use std::time::Duration;

use crate::traits::Equipment;

#[derive(Debug, PartialEq)]
pub enum Status {
    Available,
    Rented {
        customer: Customer,
        rental_duration: Duration,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub phone_number: u64,
}

impl Customer {
    pub fn new(id: u32, name: String, phone_number: u64) -> Self {
        Self {
            id,
            name,
            phone_number,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Camera {
    pub id: u32,
    pub brand: String,
    pub model: String,
    pub resolution: String,
    pub status: Status,
}

impl Camera {
    fn new(id: u32, brand: String, model: String, resolution: String, status: Status) -> Self {
        Self {
            id,
            brand,
            model,
            resolution,
            status,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Laptop {
    pub id: u32,
    pub brand: String,
    pub processor: String,
    pub ram: String,
    pub status: Status,
}

impl Laptop {
    pub fn new(id: u32, brand: String, processor: String, ram: String, status: Status) -> Self {
        Self {
            id,
            brand,
            processor,
            ram,
            status,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Projector {
    pub id: u32,
    pub brand: String,
    pub lumens: String,
    pub status: Status,
}

impl Projector {
    fn new(id: u32, brand: String, lumens: String, status: Status) -> Self {
        Self {
            id,
            brand,
            lumens,
            status,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Microphone {
    pub id: u32,
    pub brand: String,
    pub microphone_type: String,
    pub status: Status,
}

impl Microphone {
    fn new(id: u32, brand: String, microphone_type: String, status: Status) -> Self {
        Self {
            id,
            brand,
            microphone_type,
            status,
        }
    }
}

pub struct RentalShop<T: Equipment> {
    pub name: String,
    pub equipments: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub struct RentalRecord<'a, 'b, T: Equipment> {
    pub rented_equipment: &'a T,
    pub customer: &'b Customer,
    pub rental_date: String,
}

impl<'a, 'b, T: Equipment> RentalRecord<'a, 'b, T> {
    pub fn new(rented_equipment: &'a T, customer: &'b Customer, rental_date: String) -> Self {
        Self {
            rented_equipment,
            customer,
            rental_date,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RentalHistory<'a, 'b, T: Equipment> {
    pub name: String,
    pub rental_history: Vec<RentalRecord<'a, 'b, T>>
}

impl<'a, 'b, T: Equipment + std::cmp::PartialEq>  RentalHistory<'a, 'b, T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rental_history: Vec::<RentalRecord<'a, 'b, T>>::new(),
        }
    }
}
