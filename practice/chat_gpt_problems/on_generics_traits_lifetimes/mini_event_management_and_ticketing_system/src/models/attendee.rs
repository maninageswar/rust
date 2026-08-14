use crate::traits::{registrable::Registrable, identifiable::Identifiable};
use crate::registration::record::RegistrationRecord;

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
pub struct AttendeeHistory<'a, 'b, T: Registrable> {
    pub attendee: &'b Attendee,
    pub registrations: Vec<&'a RegistrationRecord<'a, 'b, T>>,
}
