// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a75a073-518c-83ee-b6c4-9036707c905a
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
