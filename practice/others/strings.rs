fn main() {
    // let a1:&str = "super";
    // let a2:&str = "man";
    // let a3:&str = a1 + a2;
    // println!("{a3}");    
    let a1 = String::from("super");
    let a2 = "man";
    let a3 = a1 + a2;
    // println!("{a1}");
    println!("{a2}");
    println!("{a3}");

    let b1 = String::from("bat");
    let b2 = String::from("man");
    let b3 = b1 + b2;
    println!("{b3}");
}