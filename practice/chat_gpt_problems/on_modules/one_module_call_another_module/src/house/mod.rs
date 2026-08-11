use crate::devices::{coffee_maker, thermostat};

pub fn start_morning_routine() {
    let coffee_maker1: coffee_maker::CoffeeMaker = coffee_maker::CoffeeMaker {
        brand: String::from("Philips"),
    };
    println!("{}", coffee_maker1.brew());

    let mut living_room_thermostat: thermostat::Thermostat = thermostat::Thermostat {
        room: String::from("living_room"),
        temp: 27,
    };
    println!("{}", living_room_thermostat.set_temp(22));
}