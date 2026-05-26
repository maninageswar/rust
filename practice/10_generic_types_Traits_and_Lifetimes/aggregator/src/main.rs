mod lib;
use lib::{NewsArticle, Summary};

fn main() {
    let article1: NewsArticle = NewsArticle {
        headline: String::from("your name is in epstine files"),
        location: String::from("nowhere"),
        author: String::from("no one"),
        content: String::from("Just kidding. I just kept the headline like that just to grab your attention"),
    };

    println!("wanna know how the famous celebrity mikel's name appeared in epstine files then click to {}", article1.summarize());
}
