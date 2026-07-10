trait AnimalBehaviour {
    fn speak(&self) -> ();

    fn make_sound(&self) {
        self.speak();
    }
}

struct Dog {
    name: String,
    breed: String,
}

impl AnimalBehaviour for Dog {
    fn speak(&self) {
        println!("WOOF");
    }
}

struct Cat {
    name: String,
    breed: String,
}

impl AnimalBehaviour for Cat {
    fn speak(&self) {
        println!("MEOW");
    }
}

fn main() {
    let dog1: Dog = Dog {
        name: String::from("raju"),
        breed: String::from("sitzu"),
    };

    let cat1: Cat = Cat {
        name: String::from("rani"),
        breed: String::from("Persian"),
    };

    dog1.make_sound();
    cat1.make_sound();
}