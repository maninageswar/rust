#[derive(Debug)]
pub struct Account {
    bank_name: String,
    account_holder_name: String,
    account_number: u64,
    pub balance: u64,
}

impl Account {
    pub fn new(bank_name: String, account_holder_name: String, account_number: u64, balance: u64,) -> Self {
        Self {
            bank_name,
            account_holder_name,
            account_number,
            balance,
        }
    }

    pub fn get_bank_name(&self) -> &String {
        &self.bank_name
    }

    pub fn get_account_holder_name(&self) -> &String {
        &self.account_holder_name
    }

    pub fn get_account_number(&self) -> &u64 {
        &self.account_number
    }

    pub fn get_balance(&self) -> &u64 {
        &self.balance
    }
}