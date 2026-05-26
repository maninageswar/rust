// fn main() {
//     let mut x = vec![1,2,3];

//     x.push(4);
//     let last = x.last().unwrap();

//     println!("the last element is {:?}", last);
// }

// fn main() {
//     let v = vec![1, 2, 3];

//     for x in &v {
//         println!("{}", x);
//         // v.push(4); // ❌ try this → error
//     }
// }

fn main() {
    let mut v = vec![1, 2, 3];
    let mut to_add = vec![];

    for x in &v {
        // println!("{}", x);
        to_add.push(4);
    }

    v.extend(to_add);

    println!("the vector after adding 4 is {:?}", v);
}
