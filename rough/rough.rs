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

// struct Item {
//     name: String,
//     price: f32,
// }

// impl Item {
//     fn test() {
//         println!("hello")
//     }
// }

// fn main() {
//     let mut item1 = Item {
//         name: String::from("hello"),
//         price: 10.0,
//     };

//     Item::test()
// }

// struct Car;

// impl Car {
//     fn get_purpose()  {
//         println!("transport");
//     }

//     fn get_color(&self) -> &'static str {
//         "red"
//     }
// }

// fn main() {
//     let car1 = Car;

//     println!("the color of the car1 is {}", car1.get_color());

//     car1.get_purpose();
// }


// fn main() {
//     let v = vec![
//         String::from("superman"),
//         String::from("batman"),
//         String::from("ironman"),
//     ];

//     let third = v[2];

//     // let third_copy = v[2];

//     println!("the vector is {:?}",v);
// }

// fn main() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         v.push(9)
//     }
// }

// fn main() {
//     let string1 = String::from("flower");
//     let string_length = string1.len();
//     for word_length in (1..string_length+1).rev() {
//         let word = &string1[..word_length];
//         println!("the word is {}",word);
//     }
    
// }

// fn main() {
//     let v: Vec<i32> = Vec::from([1, 2, 3, 4]);
//     let does_not_exist = &v[100];
//     println!("does not exits {}", does_not_exist);
// }

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//     let full_string = String::from("tic-tac-toe");

//     let s = format!("{s1}-{s2}-{s3}");
//     drop(s2);
//     println!("s is {}", s);
//     // println!("s1 is {}", s1);
//     // println!("s2 is {}", s2);
//     // println!("s3 is {}", s3);

    

//     if s == full_string {
//         println!("both strings are equal");
//     }
// }

// fn main() {
//     let s = "Здравствуйте";
//     let a = s.chars().nth(1);
//     println!("a is {:?}",a);
// }

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Yellow"), 50);
//     scores.insert(String::from("Blue"), 10);
//     println!("score of blue team is {}", scores["Blue"]);
// }

// fn main() {
//     let word1 = String::from("super");
//     println!("the word is {}", can_i_print(&word1));
// }

// fn can_i_print(word: &str) -> String {
//     if word == "super" {
//         String::from("super")
//     } else {
//         String::from("hello")
//     }
// }

// fn main() {
//     let word1 = String::from("hello");
//     let word_ref = &word1;
//     let mode = Some(word_ref);
//     if let Some(&word) = mode {
//         println!("The word is {}", word);
//     }
// }

// fn main() {
//     let v = String::from("hello");

//     let first = &v;

//     let moved = *first;
//     println!("{}", moved);
// }

// use std::collections::HashMap;

// fn main() {
//     let mut scores: HashMap<String, i32> = HashMap::new();
//     scores.insert(String::from("blue"),30);
//     scores.insert(String::from("red"),20);
//     println!("the score of the blue team is {}", scores["blue"]);
//     println!("{:#?}",scores);
// }

// fn main() {
//     let mut s = String::from("hello");
//     let Is = &s;
//     let ms = &mut s;
//     println!("the value is {}",Is);
// }


// struct User {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let mut user = User {
//         name: String::from("Nathan"),
//         age: 20,
//     };

//     let name_ref = &user.name;
//     let user_ref = &mut user;

//     user_ref.age += 1;
//     user_ref.name.push_str(" Kumar");

//     println!("{}", name_ref);

//     println!("{}", user_ref.age);
//     println!("{}", user_ref.name);
// }


// fn main() {
//     let v = vec![
//         String::from("sai"),
//         String::from("sri"),
//         String::from("shankar"),
//     ];

//     let middle_name: &String = &v[1];
//     println!("the middle name is {}", middle_name);

//     let last_name: Option<&String> = v.get(2);
//     match last_name {
//         Option::Some(name) => { println!("the last name is {}", name); },
//         Option::None => { println!("the index is out of range"); }
//     }

//     if let Some(name) = last_name {
//         println!("the last name is {}", name);
//     }
// }

// fn main() {
//     let mut v = Vec::from([1, 2, 3, 4, 5]);
//     let x = &v;

//     println!("vec x is {:?}", x);

//     for i in 6..=105 {
//         v.push(i);
//     }

//     // println!("vec x is {:?}", x);
// }

// fn main() { 
//     let mut v = vec![1, 2, 3, 4, 5]; 
//     let x = &mut v; 
//     println!("vex x before {:?}",x); 
//     for i in 6..=105 { 
//         x.push(i); 
//     } 
//     println!("vex v after {:?}",v); 
// }

// #[derive(Debug)]
// struct User {
//     id: u32,
//     name: String,
// }

// fn main() {
//     let mut v: Vec<&User> = Vec::new();
//     {
//         let u = User { id: 1, name: String::from("siri") };
//         v.push(&u)
//     }

//     println!("the vector is {:#?}", v);
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     let first = &v[0];
//     v.push(6);
//     println!("The first element is: {first}");
// }

// fn main() {
//     let name = String::from("tarun");
//     let ref_name = &name;
//     drop(name);
//     println!("the name is {}",ref_name);
// }

// fn main() {
//     let mut name = String::from("tarun");
//     let name_im_ref1 = &mut name;
//     let name_im_ref2 = &mut name;
//     // name_im_ref2.push_str("swamy");
//     println!("the name via ref is {}",name_im_ref1);
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = "world!";
//     let s3 = s1 + s2; 
//     println!("the s3 is {}", s3);
//     println!("the s2 is {}", s2);
//     println!("the first char is {}", &s1[..1]);
//     println!("the first char is {:?}", s1.get(0..1).unwrap());
// }

// fn main() {
//     let v: Vec<String> = vec![String::from("sai"), String::from("sri")];
//     let a = &v;
//     let b = &a[0];
//     let c = &a[0];
//     println!("the a is {:?}",b);
//     println!("the c is {}",c);
//     println!("the a is {:?}",a[1]);
//     println!("the a is {:?}",&a[1]);
// }

// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3, 4];
//     let a = &v;
//     println!("the a is 1 {:?}",a[0]);
//     println!("the a is 2 {:?}",&a[0]);
// }

// #[derive(Debug)]
// struct Clock {
//     hours: i32,
//     minutes: i32,
// }

// impl Clock {
//     fn new(hours: i32, minutes: i32) -> Self {
//         Self {
//             hours,
//             minutes
//         }
//     }
// }

// fn main() {
//     let c1: Clock = Clock::new(9, 54);
//     println!("the time is {:?}", c1);
// }

// fn main() {
//     for i in 1..3+1 {
//         println!("{}", i);
//     }
//     println!("{}", -12 % 24)
// }

// fn main() {
//     let s1: String = String::from("apple");
//     let s2: String = String::from("zoo");
//     if s1 > s2 {
//         println!("{} is greater", s1);
//     } else {
//         println!("{} is greater",s2)
//     }
// }


// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }

// fn main<'a>() {
//     {
//         let s = String::from("hello");
//         let r: &'a String = &s;
//     }
// }

// fn main() {
//     let r;
//     {
//         let x = 10;
//         r = &x;
//     }
//     println!("{r}");
// }

// fn main() {
//     let x = 10;
//     let r = &'a x;
//     println!("{r}");
// }

// fn longest<'a, 'b>(
//     x: &'a str,
//     y: &'b str,
// ) -> &'b str { 
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let a = String::from("hello");
//     let b = String::from("world!!!");

//     let result = longest(&a, &b);

//     println!("{result}");
// }

// fn main() {
//     let super_array: [&str; 5] = ["super"; 5];
//     println!("the super array {:#?}", super_array);

//     let mut months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
    
//     let first_quater: &[&str] = &months[0..3];
//     println!("months that belong to first quater are {:#?}", first_quater);
    
// }

// fn find_largest(list: &[i32]) -> &i32 {
//     let mut largest_number: &i32 = &list[0];
//     for number in list {
//         if number > largest_number {
//             largest_number = number
//         }
//     }
//     largest_number
// }

// fn main() {
//     let numbers: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
//     println!("the largest number in the array {:?} is {}", numbers, find_largest(&numbers));
// }

// use std::cmp::PartialOrd;

// fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest_number: &T = &list[0];
//     for number in list {
//         if number > largest_number {
//             largest_number = number;
//         }
//     }
//     largest_number
// }

// fn main() {
//     let numbers: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
//     println!("the largest number in the array {:?} is {}", numbers, find_largest(&numbers));

//     let charecters: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
//     println!("the largest charecter in the array {:?} is {}", charecters, find_largest(&charecters));
// }

// trait Summary {
//     fn summarize(&self) -> String;
// }

// struct NewsArticle {
//     content: String,
//     location: String,
//     author: String,
// }

// impl NewsArticle {
//     fn get_news_article_content(self: &Self) -> &String {
//         &self.content
//     }
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         let content: &String = &self.content;
//         format!("{}, by {} from ({})", &self.content, &self.author, &self.location)
//     }
// }

// fn main() {
//     let article1: NewsArticle = NewsArticle {
//         content: String::from("anthropic released it's most powerful model called mythos"),
//         location: String::from("India"),
//         author: String::from("superman")
//     };

//     println!("the content of the article1 is: {}", article1.get_news_article_content());
//     println!();
//     println!("the content of the article1 is: {}", article1.content);
//     println!();
//     println!("the summary of the article1 is {}", article1.summarize());
//     println!();
//     println!("the content of the article1 is: {}", article1.content);
// }

// fn main() {
//     let word1 = String::from("apple");
//     let word2 = &word1;
//     let word3 = word1;
//     println!("word2 is {}", word2);
// }