use crate::models::attendee::Attendee;
use crate::traits::registrable::Registrable;

#[derive(Debug)]
pub struct RegistrationRecord<'a, 'b, T: Registrable> {
    pub id: usize,
    pub event: &'a T,
    pub attendee: &'b Attendee,
    registration_date: String,
}

impl<'a, 'b, T: Registrable> RegistrationRecord<'a, 'b, T> {
    pub fn new(id: usize, event: &'a T, attendee: &'b Attendee, registration_date: String) -> Self {
        Self {
            id,
            event,
            attendee,
            registration_date,
        }
    }
}
