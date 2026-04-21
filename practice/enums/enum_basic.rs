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
//     enum Message {
//         Quite,
//         Move {x: u8, y: u8},
//         Write(String),
//         ChangeColor(i32, i32, i32)
//     }

//     impl Message {
//         fn call(self: &Self) {
//             // match self {
//                 // Message::Write(s) => {
//                 //     println!("the word to write is {}", s);
//                 // }
//                 // _ => {}
//             // }
//             if let Message::Write(s) = self {
//                     println!("the word to write is {}",s);
//                 }
//         }
//     }

//     let m: Message = Message::Write(String::from("hello"));
//     m.call();
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quater(UsState),
// }

// fn main() {
//     let c1: Coin = Coin::Quater(UsState::Alaska);
//     println!("c1 in cents is {}",value_in_cents(c1))
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quater(state) => {
//             println!("the state to which this Quater belongs is {:?}",state);
//             25
//         },
//     }
// }

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32>  {
//         match x {
//             None => None,
//             Some(i) => Some(i+1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("the number six is {:?}",six);
//     println!("the number none is {:?}",none);
// }

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
