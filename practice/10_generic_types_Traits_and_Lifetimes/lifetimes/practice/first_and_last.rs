fn first_and_last(word: &str) -> (&str, &str) {
    (&word[..1], &word[word.len()-1..])
}

fn main() {
    let word = String::from("rust");
    let (first, last) = first_and_last(&word);
    println!("the first and last letters of the word: {} is {} and {}", word, first, last);
}