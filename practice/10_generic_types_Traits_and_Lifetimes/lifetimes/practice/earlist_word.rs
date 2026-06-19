fn earlist_word<'a>(word1: &'a str, word2: &'a str) -> &'a str {
    if &word1[..1] < &word2[..1] {
        word1
    } else {
        word2
    }
}


// fn main() {
//     let word1: String = String::from("apple");
//     let word2: String = String::from("zebra");
//     println!("the earlist word of {} and {} is {}", word1, word2, earlist_word(&word1, &word2));
// }

// different ways to make compiler throws an error for earlist_word method

// fn main() {
//     let result;
//     let word1 = String::from("apple");
//     {
//         let word2 = String::from("zebra");
//         // 'result' gets the lifetime of the shortest-lived argument (word2)
//         result = earlist_word(&word1, &word2); 
//     } // word2 is dropped here!

//     // ERROR: `word2` does not live long enough
//     println!("The earliest word is {}", result); 
// }

// fn main() {
//     let word1 = String::from("apple");
//     // String::from("zebra") is a temporary value dropped at the semicolon
//     let result = earlist_word(&word1, &String::from("zebra")); 
    
//     // ERROR: temporary value dropped while borrowed
//     println!("The earliest word is {}", result);
// }

fn main() {
    let word1 = String::from("apple");
    let word2 = String::from("zebra");
    
    let result = earlist_word(&word1, &word2);

    // We move/drop word1, rendering 'result' potentially invalid
    // let word3 = word1;

    // explination of the error that will be thrown when you uncomment the above line

    // The borrow checker looks purely at the function signature, not the function body.

    // The signature is: fn earlist_word<'a>(word1: &'a str, word2: &'a str) -> &'a str

    // This tells the compiler: "the returned reference lives as long as 'a, and 'a is constrained by both word1 and word2".

    // The borrow checker does not look inside the function to see which branch actually runs. It only sees:

    // result has lifetime 'a
    // 'a is tied to both word1 and word2
    // Therefore, result might point to word1, or it might point to word2
    // Since it can't know at compile time which one result actually points to (that depends on runtime values), it conservatively assumes both borrows must stay alive for as long as result is used.

    // This is intentional — the borrow checker is a static analysis tool. It reasons about types and lifetimes, not runtime behavior. If the signature says the output lifetime depends on both inputs, the checker enforces that both inputs remain valid and unmoved for the entire lifetime of the output.

    // ERROR: cannot move out of `word1` because it is borrowed
    println!("The earliest word is {}", result);
}

// fn main() {
//     let mut word1 = String::from("apple");
//     let word2 = String::from("zebra");
    
//     let result = earlist_word(&word1, &word2);

//     // ERROR: cannot borrow `word1` as mutable because it is also borrowed as immutable
//     word1.push_str(" sauce"); 

//     println!("The word1 is {}", result);
// }
