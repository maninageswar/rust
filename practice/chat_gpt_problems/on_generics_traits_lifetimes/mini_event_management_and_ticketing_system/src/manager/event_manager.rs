use crate::traits::identifiable::Identifiable;

#[derive(Debug)]
pub struct EventManager<T> {
    company: String,
    events: Vec<T>,
}

impl<T: Identifiable + std::fmt::Debug> EventManager<T> {
    pub fn new(company: String) -> Self {
        Self {
            company,
            //  writing events: Vec::<T>::new() (using the "turbofish" syntax) is perfectly valid Rust
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, event_id: u32) {
        self.events.retain(|event| event.get_id() != event_id);
    }

    pub fn find_event(&self, event_id: u32) -> Option<&T> {
        self.events.iter().find(|event| event.get_id() == event_id)
    }

    pub fn list_all_events(&self) {
        println!("events : {:#?}", self.events);
    }

    // pub fn register_attendee()

    // pub fn cancel_registration()
}

pub fn display_event<T: Identifiable + std::fmt::Debug>(event: &T) {
    println!("event is {:#?}", event);
}
