fn main() {
    let mut word1: String = String::from("first");
    let mut word2: String = String::from("apple");
    println!("the pig latin word is {}", conert_to_pig_latin(&mut word1));
    println!("the pig latin word is {}", conert_to_pig_latin(&mut word2));
}

fn conert_to_pig_latin(word: &mut str) -> String {
    if word.len() == 0 {
        return String::from("the string is empty hence cannot be converted to pig latin");
    }
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let first_char: char = word.chars().nth(0).unwrap();
    if vowels.contains(&first_char) {
        // word + "-hey" // won't work cuz word is of type str but we need owned string (String) for + which is Add method internally
        // word.push_str("-hey") // won't work cuz word is of type str which does not have push_str method str is not growable because it's just a slice that points to some part of the owned string
        format!("{word}-hey")
    } else {
        format!("{}-{}ay",&word[1..], first_char)
    }
}