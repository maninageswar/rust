use fornt_of_house::hosting;
use back_of_house::Breakfast::summer;

mod fornt_of_house {
    pub mod hosting {
        pub fn add_to_waitinglist() {
            println!("add_to_waitinglist mehod called")
        }
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
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
}

fn eat_at_restaurant() {
    hosting::add_to_waitinglist();
    let breakfast = Breakfast::summer(String::from("idli"));
    println!("the item for today's breakfast is {}", breakfast.item);
}