enum Shapes {
    Circle(f32),
    Rectangle {
        length: f32,
        width: f32,
    },
    Square(f32),
}

const PI: f32 = 3.141592653589793;

fn main() {
    let radius: f32 = 35.6;
    let circle = Shapes::Circle(radius);
    println!();
    println!("the area of the circle of radius: {radius} is {}", calculate_area_with_reference(&circle));
    println!("the area of the circle of radius: {radius} is {}", calculate_area(circle));
    
    let len: f32 = 53.4;
    let wid: f32 = 23.2;
    let rectangle = Shapes::Rectangle { length: len, width: wid };
    println!();
    println!("the area of the rectangle of length: {len} and width: {wid} is {}", calculate_area_with_reference(&rectangle));
    println!("the area of the rectangle of length: {len} and width: {wid} is {}", calculate_area(rectangle));

    let side: f32 = 25.0;
    let square = Shapes::Square(side);
    println!();
    println!("the area of the square of side: {side} is {}", calculate_area_with_reference(&square));
    println!("the area of the square of side: {side} is {}", calculate_area(square));
}

fn calculate_area(shape: Shapes) -> f32 {
    match shape {
        Shapes::Circle(radius) => { PI * (radius.powi(2)) },
        Shapes::Rectangle { length, width } => { length * width },
        Shapes::Square(side) => { side.powi(2) },
    }
}

fn calculate_area_with_reference(shape: &Shapes) -> f32 {
    match shape {
        Shapes::Circle(radius) => { PI * (radius.powi(2)) },
        Shapes::Rectangle { length, width } => { length * width },
        Shapes::Square(side) => { side.powi(2) },
    }
}