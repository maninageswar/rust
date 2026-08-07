use crate::bank::account::*;

pub fn cash_withdraw(bank_account: &mut Account, amount_to_withdraw: u64) {
    bank_account.balance -= amount_to_withdraw;
    println!("the amount of {} rupees has been withdrawn form your account. your current balance is {}", amount_to_withdraw, bank_account.get_balance())
}

pub fn cash_deposite(bank_account: &mut Account, amount_to_credit: u64) {
    bank_account.balance += amount_to_credit;
    println!("the amount of {} rupees has been credited to your account. your current balance is {}", amount_to_credit, bank_account.get_balance())
}