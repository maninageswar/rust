#[derive(Debug)]
struct Container<T> {
    value:T,
}

impl<T> Container<T> {
    fn new(val: T) -> Container<T> {
        Container {
            value: val,
        }
    }
    
    fn get_value(self: &Self) -> &T {
        &self.value
    }

    fn replace(self: &mut Self, value_to_replace: T) {
        self.value = value_to_replace;
    }
}


fn main() {
 let mut c = Container::new(3.2);
 println!("the value is {}", c.value);
 c.replace(4.0);
 println!("the value is {}", c.get_value());
}