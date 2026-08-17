use super::attendee::Attendee;
use crate::registration::record::{RegistrationRecord, RegistrationError};
use crate::traits::{identifiable::Identifiable, registrable::Registrable};
use std::{time::Duration, cell::Cell};

#[derive(Debug)]
pub struct Conference {
    id: u32,
    name: String,
    speaker: String,
    duration: Duration,
    capacity: usize,
    // Explination !important : if you to understand why i use Cell type just go through the explination at learnings_and_error_explinations/understand_why_and_how_to_use_Cell_type.md
    registered_count: Cell<usize>,
}

impl Conference {
    pub fn new(id: u32, name: String, speaker: String, duration: Duration, capacity: usize, registered_count: usize) -> Self {
        Self {
            id,
            name,
            speaker,
            duration,
            capacity,
            registered_count: Cell::new(registered_count),
        }
    }
}

impl Identifiable for Conference {
    /*
    error:
    error[E0449]: visibility qualifiers are not permitted here
    --> src/models/event.rs:24:5
    |
    24 |     pub fn get_id(&self) -> u32 {
    |     ^^^ help: remove the qualifier
    |
    = note: trait items always share the visibility of their trait

    For more information about this error, try `rustc --explain E0449`.

    copilot explination:
    The error occurs because in Rust, you cannot use visibility modifiers (like pub) inside a trait implementation block.

    When you implement a trait for a struct, the visibility of the methods is entirely determined by the visibility of the trait itself. If the Identifiable trait is public, its methods are automatically publicly accessible for any type that implements it.

    To fix the error, simply remove the pub keyword from the get_id method inside the impl Identifiable for Conference block.
    */
    // if you use the visibility modifier "pub" for the below method you will get the above error
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Registrable for Conference {
    fn register<'a, 'b>(&'a self, registration_history_length: usize, attendee: &'b Attendee) -> Result<RegistrationRecord<'a, 'b, Self>, RegistrationError> {
        // Explination !important : if you to understand why i use Cell type which uses get to access the registered_count value just go through the explination at learnings_and_error_explinations/understand_why_and_how_to_use_Cell_type.md
        let current_count = self.registered_count.get();
        if self.capacity > current_count {
            self.registered_count.set(current_count + 1); 
            let registration_record: RegistrationRecord<'a, 'b, Self> = RegistrationRecord::new(registration_history_length + 1, self, attendee, String::from("registration_date"));
            Ok(registration_record)
        } else {
            Err(RegistrationError::CapacityReached)
        }
    }

    fn un_register(&mut self, id: u32) -> String {
        let current_count = self.registered_count.get();
        if current_count > 0 {
            self.registered_count.set(current_count - 1);
        }
        un_registration_successful()
        // TODO: delete the registration record from the registration history
    }

    fn is_registration_available(&self) -> bool {
        self.capacity > self.registered_count.get()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

#[derive(Debug)]
pub struct Workshop {
    id: u32,
    name: String,
    instructor: String,
    capacity: usize,
    registered_count: Cell<usize>,
}

impl Workshop {
    pub fn new(id: u32, name: String, instructor: String, capacity: usize, registered_count: usize) -> Self {
        Self {
            id,
            name,
            instructor,
            capacity,
            registered_count: Cell::new(registered_count),
        }
    }
}

impl Identifiable for Workshop {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Registrable for Workshop {
    fn register<'a, 'b>(&'a self, registration_history_length: usize, attendee: &'b Attendee) -> Result<RegistrationRecord<'a, 'b, Self>, RegistrationError> {
        // Explination !important : if you to understand why i use Cell type which uses get to access the registered_count value just go through the explination at learnings_and_error_explinations/understand_why_and_how_to_use_Cell_type.md
        let current_count = self.registered_count.get();
        if self.capacity > current_count {
            self.registered_count.set(current_count + 1); 
            let registration_record: RegistrationRecord<'a, 'b, Self> = RegistrationRecord::new(registration_history_length + 1, self, attendee, String::from("registration_date"));
            Ok(registration_record)
        } else {
            Err(RegistrationError::CapacityReached)
        }
    }

    fn un_register(&mut self, id: u32) -> String {
        let current_count = self.registered_count.get();
        if current_count > 0 {
            self.registered_count.set(current_count - 1);
        }
        un_registration_successful()
        // TODO: delete the registration record from the registration history
    }

    fn is_registration_available(&self) -> bool {
        self.capacity > self.registered_count.get()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

#[derive(Debug)]
pub struct Concert {
    id: u32,
    name: String,
    artist: String,
    duration: Duration,
    capacity: usize,
    registered_count: Cell<usize>,
}

impl Concert {
    pub fn new(id: u32, name: String, artist: String, duration: Duration, capacity: usize, registered_count: usize) -> Self {
        Self {
            id,
            name,
            artist,
            duration,
            capacity,
            registered_count: Cell::new(registered_count),
        }
    }
}

impl Identifiable for Concert {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Registrable for Concert {
    fn register<'a, 'b>(&'a self, registration_history_length: usize, attendee: &'b Attendee) -> Result<RegistrationRecord<'a, 'b, Self>, RegistrationError> {
        // Explination !important : if you to understand why i use Cell type which uses get to access the registered_count value just go through the explination at learnings_and_error_explinations/understand_why_and_how_to_use_Cell_type.md
        let current_count = self.registered_count.get();
        if self.capacity > current_count {
            self.registered_count.set(current_count + 1); 
            let registration_record: RegistrationRecord<'a, 'b, Self> = RegistrationRecord::new(registration_history_length + 1, self, attendee, String::from("registration_date"));
            Ok(registration_record)
        } else {
            Err(RegistrationError::CapacityReached)
        }
    }

    fn un_register(&mut self, id: u32) -> String {
        let current_count = self.registered_count.get();
        if current_count > 0 {
            self.registered_count.set(current_count - 1);
        }
        un_registration_successful()
        // TODO: delete the registration record from the registration history
    }

    fn is_registration_available(&self) -> bool {
        self.capacity > self.registered_count.get()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

#[derive(Debug)]
pub enum Event {
    Conference,
    Workshop,
    Concert,
}

fn registration_successful() {
    println!("your registration is successful")
}

fn registration_un_successful() {
    println!("sorry the maximum capacity for this event has been reached so, your registration is un successful")
}

fn un_registration_successful() -> String {
    format!("you have successfully unregistered to the event")
}
