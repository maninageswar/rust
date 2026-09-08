use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(box_value: T) -> MyBox<T> {
        MyBox(box_value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}