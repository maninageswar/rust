mod weather;

use weather::*;

fn main() {
    println!("{}", get_weather_report());
    let animal1: Animal = Animal {
        name: String::from("dodo"),
        species: String::from("dog"),
        age: 23,
    };
    println!("tha animal is {:#?}", animal1)
}
