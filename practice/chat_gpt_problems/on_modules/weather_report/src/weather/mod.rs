mod api;
mod formatter;
mod parser;

use api::*;
use parser::*;
use formatter::*;

pub fn get_weather_report() -> String {
    format_weather(parse_weather_data_to_Weather_struct(fetch_weather_data()))
}