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

// fn main() {
//     let s = String::from("hello");
//     let r = &s;
//     let x = *r;
// }

// struct Person {
//     name: String,
// }

// fn main() {
//     let p = Person {
//         name: String::from("Nathan"),
//     };

//     let r = &p; // borrow p

//     let name = r.name;

//     println!("{}", name);
// }

// fn main() {
//     let num: i32 = 121;
//     // let s = num.to_string();

//     println!("is palindrome{}", palindrome(num));
// }

// fn palindrome(x: i32) -> bool {
//     // let s = x.to_string();
//     let t : String = x.to_string().chars().rev().collect();
//     // if s == t {
//     //    true
//     // } else {
//     //     false
//     // }
//     t == x.to_string()
// }

// fn main() {
//     let roman = String::from("VI");
//     // if roman == "VII" || roman == "VI" {
//     //     println!("ther are equal");
//     // }
//     for i in roman.chars() {
//         println!("i {}",i);
//     }
// }

// fn main() {
//     let n1: u16 = 4;
//     let n2: i32 = 8;
//     let sum = n1 as i32 + n2;
//     println!("the sum is {}",sum);
// }

// fn main() {
//     let roman = String::from("some chepri");
//     let chars: Vec<char> = roman.chars().collect();
//     // for i in 0..chars.len()-1 {
//     //     println!("i {}", chars[i]);
//     // }
//     println!("i {}", chars[chars.len()-1]);
// }

// fn main() {
//     let a: String = String::from("hello");
//     let a_ref: &String = &a;
//     let b: String = String::from("hello");
//     let b_ref: &String = &b;
//     if a_ref == b_ref {
//         println!("ref's are equal");
//     } else {
//         println!("ref's are not equal");
//     }
// }

// fn main() {
//     let x_arr: [u8; 5] = [1, 2, 3, 4, 5];
//     let y_arr: &[u8] = &x_arr;
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("name1 is {}",s2);
// }

// fn main() {
//     let mut s: String = String::from("hello");
//     modify_string(&mut s);
//     append_symbols(&mut s);
//     println!("the string after modify is {}", s);
// }

// fn modify_string(s: &mut String) {
//     s.push_str("world");
// }

// fn append_symbols(s: &mut String) {
//     s.push_str("!!!!");
// }

// fn main() {
//     let mut s: String = String::from("hello");
//     modify_string(&mut s);
//     append_symbols(&mut s);
//     println!("the string after modify is {}", s);
// }
// fn modify_string(s: &mut String) {
//     s.push_str("world");
// }
// fn append_symbols(s: &mut String) {
//     s.push_str("!!!!");
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     let x = word;

//     s.clear();

//     println!("{}", x);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn print_str(s: &String) {
//     println!("{}", s);
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;

//     let r2 = &mut s;

//     print_str(r1);
// }

// fn main() {
//     let mut x = vec![1, 2, 3];
//     x.push(4); // -> x is mutable borrow
//     let last = x.last().unwrap(); // > here last holds immutable borrow
//     println!("the last element is {:?}", last)
// }

// main.rs
// mod utils {
//     pub mod math {
//         pub fn add(a: i32, b: i32) -> i32 {
//             a + b
//         }
//     }
// }

// fn main() {
//     let result = utils::math::add(2, 3);
//     println!("{}", result);
// }

// struct Item {
//     name: String,
//     price: f32,
// }

// impl Item {
//     fn print_item_name(self: &Self) {
//         println!("the name of the item is {}", self.name);
//     }
// }

// fn main() {
//     let item1 = Item {
//         name: String::from("apple"),
//         price: 10.0,
//     };

//     let item2 = &item1;

//     let item_name = &item2.name;

//     item2.print_item_name();

//     println!("the price of the item is {}", item2.price);
//     println!("the price of the item is {}", item_name);
// }

struct Item {
    name: String,
    price: f32,
}

fn main() {
    let mut item1 = Item {
        name: String::from("hello"),
        price: 10.0,
    };

    let item2 = &mut item1;

    item2.name.push_str("world");
    item2.price = 20.0;

    println!("the item name is {}", item1.name);
    println!("the item price is {}", item1.price);
}
