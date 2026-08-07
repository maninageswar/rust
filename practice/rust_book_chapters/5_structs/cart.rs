#[derive(Debug)]
struct Item {
    name: String,
    price: f32,
}

#[derive(Debug)]
struct Cart {
    items: Vec<Item>,
}

impl Cart {
    fn add_item_to_the_cart(&mut self, item: Item) {
        self.items.push(item);
    }

    fn get_the_total_price_of_items(self: &Self) -> f32 {
        let mut total_price: f32 = 0.0;
        for item in &self.items {
            total_price += item.price;
        }
        total_price
    }
}

fn main() {
    let mut cart1 = Cart {
        items: Vec::<Item>::new(),
    };

    let item1 = Item {
        name: String::from("orange"),
        price: 30.5,
    };

    let item2 = Item {
        name: String::from("apple"),
        price: 31.5,
    };

    cart1.add_item_to_the_cart(item1);
    cart1.add_item_to_the_cart(item2);
    println!("the cart is {:#?}",cart1);
    println!("the total cart amount is {}",cart1.get_the_total_price_of_items());
}