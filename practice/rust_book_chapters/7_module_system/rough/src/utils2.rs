
pub mod utils3;

pub use utils3::printUtils3;

pub struct Breakfast {
    pub item: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(breakfast_item: String) -> Breakfast {
        Breakfast {
            item: breakfast_item,
            seasonal_fruit: String::from("mango"),
        }
    }
}

pub fn printUtils2() {
    println!("utils2");
}