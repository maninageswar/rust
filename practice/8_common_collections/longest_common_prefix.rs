fn main() {
    let vec_with_strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    println!("{}", longest_common_prefix(vec_with_strings));
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let first_word = &strs[0];
    for word_length in (1..first_word.len()+1).rev() {
        let prefix = &first_word[..word_length];
        let mut count = 0;
        for word in &strs {
            if word.starts_with(prefix) {
                count +=1 ;
            } else {
                break;
            }
        }
        if count == strs.len() {
            return prefix.to_string();
        }
    } 
    return String::new();
}