pub struct Guess {
    value : u32,
}

impl Guess {
    pub fn new(guess: u32) -> Guess {
        if guess < 1 || guess > 100 {
            panic!("The guess value should be inside 1 and 100");
        }
        Guess {
            value: guess,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

pub struct GuesString {
    value: String,
}

impl GuesString {
    pub fn new(guess: String) -> GuesString {
        GuesString {
            value: guess,
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}