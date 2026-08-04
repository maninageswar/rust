// remove the pub keyword in the below line and try to run it in lib.rs file wherever you used Duration
pub use std::time::Duration;

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

