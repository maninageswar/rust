use super::Rectangle;

const LARGER: Rectangle = Rectangle {
    width: 8,
    height: 7,
};
const SMALLER: Rectangle = Rectangle {
    width: 5,
    height: 1,
};

#[test]
pub fn larger_can_hold_smaller() {
    assert!(LARGER.can_hold(&SMALLER));
}

#[test]
pub fn smaller_cannot_hold_larger() {
    assert!(!SMALLER.can_hold(&LARGER));
}