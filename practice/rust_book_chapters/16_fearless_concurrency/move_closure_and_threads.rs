use std::thread::{self, JoinHandle};

/// Counts the characters in every word, on a separate thread.
fn count_chars_in_thread(words: Vec<String>) -> usize {
    // TODO: move `words` into a spawned thread, count, and join
    let handle: JoinHandle<usize> = thread::spawn(move || { 
        words.iter().map(|word| word.chars().count()).sum()
     });
    handle.join().unwrap()
}

/// Greets every name on its own thread, preserving input order.
fn greet_each(prefix: String, names: Vec<String>) -> Vec<String> {
    // TODO: spawn one thread per name and join them in spawn order
    let handle = thread::spawn(move || {
        names.into_iter().map(|name: String| format!("{prefix}, {name}!")).collect()
    });
    handle.join().unwrap()
}

// Example usage
fn main() {
    let words = vec!["hello".to_string(), "world".to_string()];
    println!("chars: {}", count_chars_in_thread(words));

    let names = vec!["Ada".to_string(), "Bo".to_string()];
    println!("{:?}", greet_each("Hi".to_string(), names));
}
