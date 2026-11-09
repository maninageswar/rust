trait Payment {
    fn pay(&self, amount: f64);
}

struct CreditCard;

impl Payment for CreditCard {
    fn pay(self: &Self, amount: f64) {
        println!("Paid {} using Credit Card", amount);
    }
}

struct UPI;

impl Payment for UPI {
    fn pay(self: &Self, amount: f64) {
        println!("Paid {} using UPI", amount);
    }
}


struct PayPal;

impl Payment for PayPal {
    fn pay(self: &Self, amount: f64) {
        println!("Paid {} using PayPal", amount);
    }
}

fn make_payment(payment_method: &dyn Payment, amount: f64) {
    payment_method.pay(amount);
}

fn main() {
    let payment_method1 = CreditCard;
    make_payment(&payment_method1, 2500.00);

    let payment_method2 = UPI;
    make_payment(&payment_method2, 1500.00);

    let payment_method3 = PayPal;
    make_payment(&payment_method3, 500.00);
}
