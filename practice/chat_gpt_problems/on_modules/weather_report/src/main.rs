mod weather;

use weather::*;

fn main() {
    println!("{}", get_weather_report());
}
