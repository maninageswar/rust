fn main() {
    let sentence: String = String::from("hi this a waste fellow learning rust");
    println!("the number of words in the sentence is {}",count_words(&sentence));
}

fn count_words(s: &str) -> u32 {
    let mut count: u32 = 0;
    for i in s.as_bytes() {
        if *i == b' ' {
            count += 1;
        }
    }
    count + 1
}