// fn main() {
//     let num = Some(4);

//     match num {
//         Some(x) if x % 2 == 0 => println!("The number {x} is even"),
//         Some(x) => println!("The number {x} is odd"),
//         None => (),
//     }

// }

// fn main() {
//     let x = 10;

//     let ptr = &x as *const i32;

//     unsafe {
//         println!("{}", *ptr);
//     }
// }

// fn main() {
//     let x = String::from("Hello");

//     let ptr: *const i32 = &x;

//     println!("{:p}", ptr);
// }

// fn main() {
//     let ptr: *const i32;

//     {
//         let x = 100;
//         ptr = &x;
//     }

//     // x is gone here, ptr is now dangling
//     println!("{:p}", ptr);

//     unsafe {
//         println!("{}", *ptr); // undefined behavior
//     }
// }

// static PI: f64 = 3.14;

// fn main() {
//     let a = PI;
//     let b = PI;
//     let c = &PI;
// }

// union MyData {
//     number: i32,
//     decimal: f32,
// }

// fn main() {
//     let data = MyData {
//         number: 100,
//     };

//     unsafe {
//         println!("{}", data.number);
//         println!("{}", data.decimal);
//     }
// }


// fn main() {
//     let x: i32 = 100;
//     let y: i32 = 50;
//     let ptr: *const i32 = &x;
//     unsafe {
//         println!("ptr is {}", ptr);
//     }
// }

// fn main() {
//     let mut x: i32 = 3;
//     let y = &mut x;
//     y+=5;
//     println!("{y}");
// }

// fn main() {
//     let address = 0x12345usize;

//     let ptr = address as *const i32;

//     println!("{:p}", ptr);
// }

// fn main() {
//     let address = 0x12345usize;
//     let ptr = address as *const i32;

//     unsafe {
//         println!("{}", *ptr);
//     }
// }

// fn main() {
//     let mut a:[i32; 5] = [1, 2, 3, 4, 5];
//     let b: &[i32] = &a;
//     let c: &mut [i32] = &mut a;
//     let d: &mut [i32] = &mut a;
//     println!("{:?}", b);
// }

fn main() {
    let a: i32 = 2;
    let b: &i32 = &a;
    println!("b is {}", *b);
}