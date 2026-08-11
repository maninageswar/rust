pub struct CoffeeMaker {
    pub brand: String,
}

impl CoffeeMaker {
    pub fn brew(&self) -> String {
        format!("Brewing hot coffee! from the brand {}", self.brand)
    }
}