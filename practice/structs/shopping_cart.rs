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
    fn add_item_to_cart(self: &mut Self, item: Item) {
        let x = &mut self.items;
        x.push(item);
    }

    fn get_the_total_price_of_items(&self) -> f32 {
        let mut total_price: f32 = 0.0;
        for item in &self.items {
            total_price += item.price;
        }
        total_price
    }

    fn remove_item_by_name(self: &mut Self, name: &str) {
        self.items.retain(|item| item.name != name);
    }

    fn total_items_in_cart(&self) -> usize {
        self.items.len()
    }

    // fn most_expensive_item(&self) -> Item {

    // }
}

fn main() {
    let mut user1_cart = Cart {
        items: Vec::<Item>::new(),
    };

    let item1 = Item {
        name: String::from("apple"),
        price: 20.0,
    };

    let item2 = Item {
        name: String::from("banana"),
        price: 10.0,
    };

    let item3 = Item {
        name: String::from("orange"),
        price: 15.0,
    };

    user1_cart.add_item_to_cart(item1);
    user1_cart.add_item_to_cart(item2);
    user1_cart.add_item_to_cart(item3);

    println!("the cart before removing the item with name orange is {:#?}",user1_cart);
    user1_cart.remove_item_by_name("orange");
    println!("the total price of all the items in user's cart after removing a item is {}",user1_cart.get_the_total_price_of_items());

    // total number of items in the cart
    println!("the number of items in the cart is {}", user1_cart.total_items_in_cart());
}