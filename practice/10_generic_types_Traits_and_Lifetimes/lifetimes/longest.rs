// the programe won't cuz it expects lifetime parameter to be added
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let x = String::from("hello");
//     let y = String::from("sai");
//     println!("the longest string is {}", longest(&x, &y));
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s2 = String::from("short");

    let result;

    {
        let s1 = String::from("long string");

        result = longest(&s1, &s2);
    }

    println!("{}", result);
}