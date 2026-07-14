mod test_rec;

mod common_tests {
    use super::{add_two, Rectangle};

    #[test]
    fn test_add_two() {
        let five: i64 = add_two(3);
        assert_eq!(five, 5);
    }

    #[test]
    fn should_be_equal() {
        let rec1: Rectangle = Rectangle {
            width: 20,
            height: 10
        };

        let rec2: Rectangle = Rectangle::new(20, 10);
        assert_eq!(rec1, rec2);
    }
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height
        }
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}

fn add_two(a: i64) -> i64 {
    a + 2
}