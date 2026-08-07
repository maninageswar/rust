fn identity<T>(item: T) -> T {
    item
}

struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
    let a = identity(3);
    let b = identity("hello");
    let c = identity(6.0);
    let d = identity('a');
    println!("the values a, b, c, d are {}, {}, {}, {}", a, b, c, d);

    let pair1: Pair<f32> = Pair {
        first: 5.0,
        second: 2.9,
    };
    println!("the first is {}", pair1.first);
}