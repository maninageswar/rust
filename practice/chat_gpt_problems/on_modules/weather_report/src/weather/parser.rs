pub struct Weather {
    city: String,
    temperature: String,
    condition: String,
}

impl Weather {
    fn new(city: String, temperature: String, condition: String) -> Self {
        Self {
            city,
            temperature,
            condition,
        }
    }

    pub fn get_city(&self) -> &String {
        &self.city
    }

    pub fn get_temperature(&self) -> &String {
        &self.temperature
    }

    pub fn get_condition(&self) -> &String {
        &self.condition
    }
    
}

pub fn parse_weather_data_to_Weather_struct(weather_data: String) -> Weather {
    let weather_in_vec: Vec<&str> = weather_data.split(',').collect();
    let weather: Weather = Weather::new(
        weather_in_vec.get(0).unwrap_or(&"").to_string(),
        weather_in_vec.get(1).unwrap_or(&"").to_string(),
        weather_in_vec.get(2).unwrap_or(&"").to_string()
    );
    weather
}