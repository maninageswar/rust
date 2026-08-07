// run cargo run to see the error that exist in utils.rs. comment the below line and run cargo run then you won't see the compiler error of utils.rs file
// this tells us that rust only 
// mod utils;

mod utils2;
use utils2::Breakfast;
use utils2::printUtils2;
use utils2::utils3::printUtils3;


fn main() {
    println!("Hello, world!");
    let breakfast = Breakfast::summer(String::from("idli"));
    println!("the item for today's breakfast is {}", breakfast.item);
    printUtils2();
    printUtils3();
}
