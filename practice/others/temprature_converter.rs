use std::io;

fn main() {

    println!("enter CF in caps if you want to convert temp form celcius to fahrenheit");

    println!("enter FC in caps if you want to convert temp form fahrenheit to celcius");

    let mut user_option = String::new();

    io::stdin().read_line(&mut user_option).expect("Failed to read line");

    let user_option = user_option.trim();

    if user_option == "CF" {

        println!("enter temparature in celcius");

        let temp_in_celcius: f64 = get_the_user_input();

        let temp_in_fahrenheit:f64 = (9.0/5.0 * temp_in_celcius) + 32.0;

        println!("temparature in fahrenheit is : {temp_in_fahrenheit}");

    } else if user_option == "FC" {
         println!("enter temparature in fahrenheit");

        let temp_in_fahrenheit: f64 = get_the_user_input();

        let temp_in_celcius: f64 = 5.0/9.0*(temp_in_fahrenheit - 32.0);

        println!("temparature in celcius is : {temp_in_celcius}");

    }
}

fn get_the_user_input()->f64 {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let temp: f64 = user_input
        .trim()
        .parse()
        .expect("Invalid number");

    temp
}