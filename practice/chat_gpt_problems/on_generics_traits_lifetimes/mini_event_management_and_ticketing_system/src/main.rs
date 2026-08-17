// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a676f87-8cac-83e8-aeef-e3aaa41bae98

use mini_event_management_and_ticketing_system::{
    models::{
        attendee::Attendee,
        event::{Conference, Event},
    },
    registration::{history::RegistrationHistory, record::{RegistrationRecord, RegistrationError}},
    traits::registrable::Registrable,
};
use std::time::Duration;

fn main() {
    let mut conference1: Conference = Conference::new(
        1,
        String::from("Rust Conference 2026"),
        String::from("Graydon Hoare"),
        Duration::from_secs((3 * 3600) + (2 * 60) + 7),
        12,
        0,
    );

    let attendee1: Attendee =
        Attendee::new(1, String::from("raju"), String::from("raju@gmail.com"));

    let attendee2: Attendee =
        Attendee::new(1, String::from("nathan"), String::from("nathan@gmail.com"));
    
    let attendee3: Attendee =
        Attendee::new(1, String::from("naveen"), String::from("naveen@gmail.com"));

    let mut registration_history_for_conferences: RegistrationHistory<'_, '_, Conference> =
        RegistrationHistory {
            event_type: Event::Conference,
            registration_history: Vec::<RegistrationRecord<'_, '_, Conference>>::new(),
        };

    let registration_record_of_attendee1_for_conference1: Result<RegistrationRecord<'_, '_, Conference>, RegistrationError> =
        conference1.register(registration_history_for_conferences.registration_history.len(), &attendee1);

    registration_history_for_conferences.add_registration(registration_record_of_attendee1_for_conference1.expect("registration was not successful to add this record into registration history of conferences"));

    let registration_record_of_attendee2_for_conference1: Result<RegistrationRecord<'_, '_, Conference>, RegistrationError> =
        conference1.register(registration_history_for_conferences.registration_history.len(), &attendee2);

    registration_history_for_conferences.add_registration(registration_record_of_attendee2_for_conference1.expect("registration was not successful to add this record into registration history of conferences"));

    let registration_record_of_attendee3_for_conference1: Result<RegistrationRecord<'_, '_, Conference>, RegistrationError> =
        conference1.register(registration_history_for_conferences.registration_history.len(), &attendee3);

    registration_history_for_conferences.add_registration(registration_record_of_attendee3_for_conference1.expect("registration was not successful to add this record into registration history of conferences"));

    registration_history_for_conferences.display_history();
}
