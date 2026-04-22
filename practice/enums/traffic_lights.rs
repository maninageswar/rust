enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light_shown = TrafficLight::Red;
    show_action(light_shown);
}

fn show_action(light: TrafficLight) {
    match light {
        TrafficLight::Red => { println!("stop"); },
        TrafficLight::Yellow => { println!("ready to go"); },
        TrafficLight::Green => { println!("go"); },
    }
}