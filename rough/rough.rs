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

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     let w=dbg!(rect1);
//     let r= w.width;
//     // let j = if true { 10 } else { 20 };
//     // println!("{j}");
// }

// fn main() {
//     for i in 1..4 {
//         print!("{} ",i);
//     }
// }

// fn show_name(n: &String) {
    
//     println!("the name super is {}",n);
// }

// fn main() {
//     let mut x: u32 = 1;
//     println!("the number before increment is {}",x);
//     increase_x(&mut x);
//     println!("the number after increment is {}",x);
// }

// fn increase_x(x: &mut u32) {
//     *x = *x + 1;
// }


// important run the coode and understand the errors

// fn main() {
//     let s = String::from("hello");
//     let r1 = &s;
//     // let r2 = &s;
//     let k = *r1 + " world";
//     println!("sum of and is {}",k);
//     println!("r1 is {}",r1)
// }

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}",r1);
}