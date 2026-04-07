// fn main() {
//     struct User {
//         name: String,
//         email: String,
//         age: u32,
//         active: bool,
//     }

//     let user1: User = User {
//         name: String::from("sai ram"),
//         email: String::from("someemail@gmail.com"),
//         age: 25,
//         active: true,
//     };

//     let user2 = User {
//         name: String::from("naveen"),
//         email: user1.email.clone(),
//         ..user1
//     };

//     println!("usesr name is {}", user1.email);
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}",origin.0);
}