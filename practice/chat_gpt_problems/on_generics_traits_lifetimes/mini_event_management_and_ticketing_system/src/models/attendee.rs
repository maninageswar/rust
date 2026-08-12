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
