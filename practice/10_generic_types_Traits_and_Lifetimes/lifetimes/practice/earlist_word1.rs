fn earlist_word<'a>(word1: &'a str, word2: &'a str) -> &'a str {
    if &word1[..1] < &word2[..1] {
        word1
    } else {
        word2
    }
}

// fn earlist_word(word1: &str, word2: &str) -> i32 {
//     return 3;
// }

fn main() {
    let mut word1 = String::from("apple");
    let word2 = String::from("zebra");
    
    let result = earlist_word(&word1, &word2);
    word1.push_str(" sauce"); 

    println!("The word1 is {}", result);
}