use crate::weather::parser::*;

pub fn format_weather(weather: Weather) -> String {
    format!("
    Weather Report

    City: {}
    Temperature: {}°C
    Condition: {}
    ", weather.get_city(), weather.get_temperature(), weather.get_condition())
}