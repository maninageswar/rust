#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1: Point<i32> = Point {
        x: 3,
        y: -2,
    };
    println!("the point is {:?}", p1);
    println!("the x coordinate of the point {:?} is {}", p1, p1.x());
    // error for the below line : 'no method named `distance_from_origin` found for struct `Point<i32>` in the current scope' why? cuz distance_from_origin method is only implemented for the type f32
    // println!("the distance for the origin to the point {:?} is {}", p1, p1.distance_from_origin());

    let p2: Point<f32> = Point {
        x: 3.0,
        y: -2.3,
    };
    println!("the distance for the origin to the point {:?} is {}", p2, p2.distance_from_origin());
}