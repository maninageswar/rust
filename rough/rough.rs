// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// fn main() {
//     let rectangle3: Rectangle = Rectangle {
//         length: 20,
//         width: 20
//     };
//     let l = rectangle3.length;
//     let w = rectangle3.width;
//     let x = area3(rectangle3);
//     println!("the area of a rectangle of length {} and width {} is {}", l, w, x);
// }

// fn area3(rec: Rectangle)-> u32 {
//     rec.length * rec.width
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let w=dbg!(rect1);
    let r= w.width;
    // let j = if true { 10 } else { 20 };
    // println!("{j}");
}