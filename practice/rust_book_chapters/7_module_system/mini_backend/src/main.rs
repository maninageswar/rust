mod auth;
mod booking;

use booking::{create::create_booking, cancel::calcel_booking};

fn main() {
    println!("from main.rs file");
    auth::login();
    auth::logout();
    create_booking();
    calcel_booking();
}
