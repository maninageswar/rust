use crate::registration::record::RegistrationRecord;
use crate::models::attendee::Attendee;

pub trait Registrable: Sized {
    fn register<'a, 'b>(&'a mut self, registration_history_length: usize, attendee: &'b Attendee) -> Option<RegistrationRecord<'a, 'b, Self>>;

    fn un_register(&mut self, id: u32) -> String;

    fn is_registration_available(&self) -> bool;

    fn capacity(&self) -> usize;
}