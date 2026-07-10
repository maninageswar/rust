#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

// impl<T> Pair<T> {
//     fn new(x: T, y: T) ->Self {
//         Self { x, y }
//     }
// }

// impl<T: std::cmp::PartialOrd> Pair<T> {
//     fn largest(&self) -> &T {
//         if self.x >= self.y {
//             &self.x
//         } else {
//             &self.y
//         }
//     }
// }

// impl<T: Copy> Pair<T> {
//     fn swap(&mut self) {
//         let temp = self.x;
//         self.x = self.y;
//         self.y = temp;
//     }
// }

// Note: here both the above commented code and below code work for this case but, if you want to understand the difference 
// read this chat at https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
// and check the code in test.rs file in the same folder

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn largest(&self) -> &T where T: std::cmp::PartialOrd {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    fn swap(&mut self) where T: Copy {
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }
}

// impl<T> Pair<T> {
//     fn new<T>(x: T, y: T) ->Self {
//         Self { x, y }
//     }

//     fn largest<T: std::cmp::PartialOrd>(&self) -> &T {
//         if self.x >= self.y {
//             &self.x
//         } else {
//             &self.y
//         }
//     }

//     fn swap<T: Copy>(&mut self) {
//         let temp = self.x;
//         self.x = self.y;
//         self.y = temp;
//     }
// }

fn main() {
    let mut p1: Pair<f32> = Pair::new(5.6, 0.5);
    print!("swap of pair ({},{})", p1.x, p1.y);
    p1.swap();
    println!(" is ({},{})", p1.x, p1.y);

    println!();

    let mut p2: Pair<char> = Pair::new('m', 'n');
    print!("swap of pair ({},{})", p2.x, p2.y);
    p2.swap();
    println!(" is ({},{})", p2.x, p2.y);

    println!();

    println!("the greatest of p2 ({},{}) is {}", p2.x, p2.y, p2.largest());

    println!();

    let mut p3: Pair<String> = Pair::new(String::from("raju"), String::from("rani"));
    println!("the greatest of p3 ({},{}) is {}", p3.x, p3.y, p3.largest());
}