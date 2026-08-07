fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let x = &mut v;
    println!("vex x before {:?}",x);
    for i in 6..=105 {
        v.push(i);
    }
    // println!("vex v after {:?}",x);
}





// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     let x = &mut v;
//     println!("vex x before {:?}",x);
//     let k = &x;
//     for i in 6..=105 {
//         x.push(i);
//     }
//     println!("vex v after {:?}",x);
//     println!("vex v after {:?}",k);
// }