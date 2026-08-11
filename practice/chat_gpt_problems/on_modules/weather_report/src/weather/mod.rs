mod api;
mod formatter;
mod parser;
mod test;

use api::*;
use parser::*;
use formatter::*;
pub use test::*;

pub fn get_weather_report() -> String {
    format_weather(parse_weather_data_to_Weather_struct(fetch_weather_data()))
}