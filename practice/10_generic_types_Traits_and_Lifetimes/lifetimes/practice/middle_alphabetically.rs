fn middle_alphabetically<'a, 'b>(w1: &'a str, w2: &'a str, w3: &'b str) -> &'a str {
    if (w1 >= w2 && w1 <= w3) || (w1 <= w2 && w1 >= w3) {
        w1
    } else if (w2 >= w1 && w2 <= w3) || (w2 <= w1 && w2 >= w3) {
        w2
    } else {
        w2
    }
}

fn main() {
    let word1 = String::from("mango");
    let word2 = String::from("zebra");
    let word3 = String::from("apple");

    let middle_word: &str = middle_alphabetically(&word1, &word2, &word3);
    println!("the middle word is {}", middle_word);
}