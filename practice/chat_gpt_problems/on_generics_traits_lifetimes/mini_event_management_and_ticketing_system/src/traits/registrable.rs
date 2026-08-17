use crate::models::attendee::Attendee;
use crate::registration::record::{RegistrationRecord, RegistrationError};

pub trait Registrable: Sized {
    fn register<'a, 'b>(&'a self, registration_history_length: usize, attendee: &'b Attendee) -> Result<RegistrationRecord<'a, 'b, Self>, RegistrationError>;

    fn un_register(&mut self, id: u32) -> String;

    fn is_registration_available(&self) -> bool;

    fn capacity(&self) -> usize;
}
