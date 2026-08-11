pub struct Thermostat {
    pub room: String,
    pub temp: i32,
}

impl Thermostat {
    pub fn set_temp(&mut self, temp: i32) -> String {
        self.temp = temp;
        format!("your {} temperature has been set to {} degrees.", self.room, self.temp)
    }
}