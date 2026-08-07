mod bank;

use bank::{account::*, transaction::*};

fn main() {
    let mut bank_account1: Account = Account::new(
        String::from("HDFC"),
        String::from("shankar"),
        98765433,
        1000,
    );
    println!("before transaction balance: {}", bank_account1.get_balance());

    cash_deposite(&mut bank_account1, 500);
    cash_withdraw(&mut bank_account1, 100);
    cash_deposite(&mut bank_account1, 2000);
}
