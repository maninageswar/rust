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

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &mut s;
//     println!("{}",r1);
// }

// fn main() {
//     #[derive(Debug)]
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home: IpAddr = IpAddr::V4(127, 0, 0, 1);
//     let loopback: IpAddr = IpAddr::V6(String::from("::1"));
//     println!("Ip address of home is {:#?}", home);
// }

// fn main() {
//     #[derive(Debug)]
//     struct Ipv4Addr {
//         // --snip--
//     }
//     #[derive(Debug)]
//     struct Ipv6Addr {
//         // --snip--
//     }

//     #[derive(Debug)]
//     enum IpAddr {
//         V4(Ipv4Addr),
//         V6(Ipv6Addr),
//     }
//     let home: IpAddr = IpAddr::V4((127, 0, 0, 1));
//     let loopback: IpAddr = IpAddr::V6(String::from("::1"));
//     println!("Ip address of home is {:#?}", home);
// }

// fn main() {
//     let x: String = String::from("hello world");
//     let y: &String = &x;
//     println!("the value of y is {}",y);
//     println!("the value of y is {:p}",y);
// }

// fn main() {
//     let value = get_value();
//     println!("the value is {}",value);
// }

// fn get_value() -> &i32 {
//     let x = 10;
//     &x
// }

// fn main() {
//     let mut x: String = String::from("hello");
//     let mut y = &mut x;
//     let z = &mut y;
//     z.push_str(", world!");
//     println!("the value of x is {}",x);
//     println!("the value of y is {}",y);
//     // println!("the value of z is {}",z);
// }

// struct Person {
//     name: String,
// }

// fn main() {
//     let p = Person {
//         name: String::from("Nathan"),
//     };

//     let r = &p; // borrow p

//     let name = r.name; // ❌ ERROR: cannot move out of borrowed content

//     println!("{}", name);
// }

fn main() {
    let s = String::from("hello");
    let r = &s;
    let x = *r;
}
