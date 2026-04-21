fn main() {
    // let sentence: &str = "hi there my name is mani nageswar tz";
    let sentence: String = String::from("qhi there my name is mani nageswar tz");
    // let c = sentence.chars().enumerate(); // won't work
    // TO LEARN : try using the above line insted of below and understnd why it will fail
    let c: Vec<(usize,char)> = sentence.chars().enumerate().collect();
    let mut found = false;
    for (i_index,i) in &c {
        for (j_index,j) in &c  {
            if i == j && i_index != j_index {
                break;
            }
            if *j_index == sentence.len() - 1 {
                // TO LEARN : if you use just j_index it will fail cuz j_index(it's a ref of a value of type usize) is of type &usize but, sentence.len() - 1
                // is of type usize so there is type mismatch in comparing them so it will fail so the sol is to access the value that is stored in
                // j_index, you can access the value by *j_index 
                println!("first non repeating char is {}", i);
                found = true;
            }
        }
        if found {
            break;
        }
    }
}

// fn main() {
//     let n1 = 10;
//     let n2 = &n1;

//     if n1 == *n2 {
//         println!("they are equal")
//     }
// }