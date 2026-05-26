struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.length * self.width
    }

    fn can_hold(self: &Self, rect2_ref: &Rectangle) -> bool {
        if self.length >= rect2_ref.length && self.width >= rect2_ref.width {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let rectangle1: Rectangle = Rectangle {
        length: 20,
        width: 30
    };

    let rectangle2: Rectangle = Rectangle {
        length: 10,
        width: 20
    };
    
    println!("rectangle1 can hold rectangle2 : {}", rectangle1.can_hold(&rectangle2));
}

