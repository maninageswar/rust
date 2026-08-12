use std::time::Duration;
use crate::traits::{identifiable::Identifiable, registrable::Registrable};

#[derive(Debug)]
pub struct Conference {
    id: u32,
    name: String,
    speaker: String,
    duration: Duration,
    capacity: usize,
    registered_count: usize,
}

impl Conference {
    pub fn new(id: u32, name: String, speaker: String, duration: Duration, capacity: usize, registered_count: usize) -> Self {
        Self {
            id,
            name,
            speaker,
            duration,
            capacity,
            registered_count,
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
    fn register(&mut self) -> String {
        if self.capacity > self.registered_count {
            self.registered_count += 1;
            registration_successful()
        } else {
            registration_un_successful()
        }
        
    }

    fn un_register(&mut self) -> String {
        self.registered_count -= 1;
        un_registration_successful()
    }

    fn is_registration_available(&self) -> bool {
        self.capacity > self.registered_count
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
    registered_count: usize,
}

impl Workshop {
    pub fn new(id: u32, name: String, instructor: String, capacity: usize, registered_count: usize) -> Self {
        Self {
            id,
            name,
            instructor,
            capacity,
            registered_count,
        }
    }
}

impl Identifiable for Workshop {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Registrable for Workshop {
    fn register(&mut self) -> String {
        if self.capacity > self.registered_count {
            self.registered_count += 1;
            registration_successful()
        } else {
            registration_un_successful()
        }
        
    }

    fn un_register(&mut self) -> String {
        self.registered_count -= 1;
        un_registration_successful()
    }

    fn is_registration_available(&self) -> bool {
        self.capacity > self.registered_count
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
    registered_count: usize,
}

impl Concert {
    pub fn new(id: u32, name: String, artist: String, duration: Duration, capacity: usize, registered_count: usize) -> Self {
        Self {
            id,
            name,
            artist,
            duration,
            capacity,
            registered_count,
        }
    }
}

impl Identifiable for Concert {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Registrable for Concert {
    fn register(&mut self) -> String {
        if self.capacity > self.registered_count {
            self.registered_count += 1;
            registration_successful()
        } else {
            registration_un_successful()
        }
        
    }

    fn un_register(&mut self) -> String {
        self.registered_count -= 1;
        un_registration_successful()
    }

    fn is_registration_available(&self) -> bool {
        self.capacity > self.registered_count
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


fn registration_successful() -> String {
    format!("your registration is successful")
}

fn registration_un_successful() -> String {
    format!("sorry the maximum capacity for this event has been reached so, your registration is un successful")
}

fn un_registration_successful() -> String {
    format!("you have successfully unregistered to the event")
}
