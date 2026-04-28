const MAXIMUM_TRANSFER_LIMIT_OF_UPI_PER_DAY: f32 = 1_00_000.0;

enum PaymentMethod {
    Cash(f32),
    Card { number: u32, amount: f32 },
    UPI { id: String, amount: f32 },
}

impl PaymentMethod {
    fn get_amount(&self) -> &f32 {
        match self {
            PaymentMethod::Cash(amount) => amount,
            PaymentMethod::Card { number, amount } => amount,
            PaymentMethod::UPI { id, amount } => amount,
        }
    }

    fn is_amount_greater_than_transfer_limit(
        payment_method_amount: &f32,
        transfer_limit: &f32,
    ) -> bool {
        return payment_method_amount > transfer_limit;
    }

    fn process_payment(&self) {
        println!();
        match self {
            PaymentMethod::Cash(amount) => {
                println!("the cash of {} will be collected at the time of product delivery. thank you for shopping", amount);
            }
            PaymentMethod::Card { number, amount } => {
                println!("the transaction of rupees {} is done successfully through the card. thank you for shopping", amount);
            }
            PaymentMethod::UPI { id, amount } => {
                // if !self::is_amount_greater_than_transfer_limit(self.get_amount(), amount) {
                // check why it throws error for above line but not for below line
                // short answer self --> instance(like which ever instance you call with)
                // Self --> actual type which is PaymentMethod here
                if !PaymentMethod::is_amount_greater_than_transfer_limit(
                    self.get_amount(),
                    &MAXIMUM_TRANSFER_LIMIT_OF_UPI_PER_DAY,
                ) {
                    println!("the UPI transaction of rupees {} is done successfully. thank you for shopping", amount);
                } else {
                    println!("sorry transation amount: {} is greater than maximum transfer limit of UPI per day: {}", amount, MAXIMUM_TRANSFER_LIMIT_OF_UPI_PER_DAY);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    price: f32,
}

fn main() {
    let item1 = Item {
        name: String::from("MacBook M5 Pro"),
        price: 249_900.00,
    };

    let item2 = Item {
        name: String::from("Apple Watch Series 11"),
        price: 46_900.00,
    };

    let item3 = Item {
        name: String::from("Iphone 17 Pro Max"),
        price: 1_69_900.00,
    };

    let item4 = Item {
        name: String::from("Apple Air Pods Pro 3"),
        price: 25_900.00,
    };

    let payment_method_for_item1 = PaymentMethod::UPI {
        id: String::from("12er34"),
        amount: item1.price,
    };
    payment_method_for_item1.process_payment();

    let payment_method_for_item2 = PaymentMethod::UPI {
        id: String::from("12er34"),
        amount: item2.price,
    };
    payment_method_for_item2.process_payment();

    let payment_method_for_item3 = PaymentMethod::Card {
        number: 1234,
        amount: item3.price,
    };
    payment_method_for_item3.process_payment();

    let payment_method_for_item4 = PaymentMethod::Cash(item4.price);
    payment_method_for_item4.process_payment();
}
