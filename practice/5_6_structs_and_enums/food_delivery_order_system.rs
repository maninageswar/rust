const DRONE_MAX_CARRY_WEIGHT: f32 = 1.0;
const DRONE_MAX_TRAVEL_DISTANCE: f32 = 25.0;

enum DeliveryType {
    Pickup,
    StandardDelivery { address: String, distance_km: f32 },
    DroneDelivery { address: String, distance_km: f32, order: FoodOrder  }
}

impl DeliveryType {
    fn process_delivery(&self) {
        match self {
            DeliveryType::Pickup => { println!("your order is ready please collect it ") },
            DeliveryType::StandardDelivery{ address, distance_km } => { println!("your order will be deliverd at {} which is {} km ", address, distance_km) },
            DeliveryType::DroneDelivery { address, distance_km, order } => {
                if Self::is_order_weight_less_than_max_drone_carry_weight(&order) {
                    println!("your order will be deliverd through drone as quick as possible")
                } else {
                    println!("your order cannot be deliverd through drone as the weight of the order is more than drone max carry weight")
                }
            }
        }
    }

    fn is_order_weight_less_than_max_drone_carry_weight(order: &FoodOrder) -> bool {
        if order.get_total_weight_of_the_order() <= DRONE_MAX_CARRY_WEIGHT {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone)]
struct FoodItem {
    name: String,
    weight: f32,
    price: f32,
}

#[derive(Debug)]
struct FoodOrder {
    restaurant_name: String,
    cart: Vec<FoodItem>,
}

impl FoodOrder {
    fn add_item_to_cart(&mut self , item: FoodItem) {
        // (&mut self.cart).push(item);
        // both the above line and below line works. the above one is a explicit one and below one is implicit
        self.cart.push(item);
    }

    fn remove_item_from_the_cart(&mut self , item: FoodItem) {
        self.cart.retain(|foodItem| foodItem.name != item.name);
    }

    fn get_total_price_of_the_order(self: &Self) -> f32 {
        let mut total_price: f32 = 0.0;
        for item in &self.cart {
            total_price += item.price;
        }
        total_price
    }

    fn get_total_weight_of_the_order(self: &Self) -> f32 {
        let mut total_weight: f32 = 0.0;
        for item in &self.cart {
            total_weight += item.weight;
        }
        total_weight
    }
}

fn main() {
    let item1 = FoodItem {
        name: String::from("idli"),
        weight: 0.5,
        price: 40.0,
    };

    let item2 = FoodItem {
        name: String::from("dosa"),
        weight: 0.9,
        price: 80.0,
    };

    let item3 = FoodItem {
        name: String::from("gulabjamun"),
        weight: 0.4,
        price: 30.0,
    };

    let item4 = FoodItem {
        name: String::from("milkshake"),
        weight: 0.4,
        price: 20.0,
    };

    let mut order1 = FoodOrder {
        restaurant_name: String::from("Meghana Foods"),
        cart: Vec::<FoodItem>::new(),
    };

    order1.add_item_to_cart(item1);
    order1.add_item_to_cart(item2);
    order1.add_item_to_cart(item3);
    order1.add_item_to_cart(item4.clone());

    println!("the order 1 is {:#?}", order1);
    println!("the total price of the order 1 is {}", order1.get_total_price_of_the_order());
    println!();
    order1.remove_item_from_the_cart(item4);
    println!("after item4 removed");
    println!("the order 1 is {:#?}", order1);
    println!("the total price of the order 1 is {}", order1.get_total_weight_of_the_order());

    let delivery1 = DeliveryType::DroneDelivery{
        address: String::from("marthahalli"),
        distance_km: 3.0,
        order: order1,
    };
    delivery1.process_delivery()
}