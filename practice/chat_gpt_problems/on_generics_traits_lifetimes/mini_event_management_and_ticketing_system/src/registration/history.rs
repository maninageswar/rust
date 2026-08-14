use std::fmt::Debug;

use crate::models::event::Event;
use crate::traits::{registrable::Registrable, identifiable::Identifiable};
use super::record::RegistrationRecord;

#[derive(Debug)]
pub struct RegistrationHistory<'a, 'b, T: Registrable> {
    event_type: Event,
    registration_history: Vec<RegistrationRecord<'a, 'b, T>>
}

impl <'a, 'b, T: Registrable + Identifiable + Debug> RegistrationHistory<'a, 'b, T> {
    pub fn add_registration(&mut self, registration_record: RegistrationRecord<'a, 'b, T>) {
        self.registration_history.push(registration_record);
    }

    pub fn remove_registration(&mut self, registration_record_id: usize) {
        self.registration_history.retain(|record| record.id != registration_record_id);
    }

    pub fn find_registrations_by_event_id(&self, event_id:u32) -> Vec<&RegistrationRecord<'a, 'b, T>>  {
        self.registration_history.iter().filter(|registration_record| registration_record.event.get_id() == event_id).collect()
    }

    pub fn find_registrations_by_attendee_id(&self, attendee_id: u32) -> Vec<&RegistrationRecord<'a, 'b, T>> {
        self.registration_history.iter().filter(|registration_record| registration_record.attendee.get_id() == attendee_id).collect()
    }

    pub fn count_registrations(&self) -> usize {
        self.registration_history.len()
    }

    pub fn display_history(&self) {
        println!("RegistrationHistory : {:#?}", self);
    }
}