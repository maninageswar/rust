fn main() {
    let word: String = String::from("hello word");
    let mut vowel_count: u32 = 0;
    let vowels:[char;5] = ['a','e','i','o','u'];
    for letter in word.chars() {
        if vowels.contains(&letter) {
            vowel_count += 1;
        } 
    }
    println!("number of vowels in the string {} is {}",word, vowel_count);
}