use crate::registration::record::RegistrationRecord;
use crate::traits::{identifiable::Identifiable, registrable::Registrable};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Attendee {
    id: u32,
    name: String,
    email: String,
}

impl Attendee {
    pub fn new(id: u32, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

impl Identifiable for Attendee {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
pub struct AttendeeHistory<'a, 'b, 'c, T: Registrable> {
    pub attendee: &'b Attendee,
    pub registrations: Vec<&'c RegistrationRecord<'a, 'b, T>>,
}

impl<'a, 'b, 'c, T: Registrable + Debug> AttendeeHistory<'a, 'b, 'c, T> {
    pub fn total_registrations(&self) -> usize {
        self.registrations.len()
    }

    pub fn display_registrations(&self) {
        println!("Attendee Registrations: {:#?}", self.registrations)
    }
}
