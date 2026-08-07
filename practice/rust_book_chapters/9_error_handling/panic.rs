// fn main() {
//     a();
// }

// fn a() {
//     b();
// }

// fn b() {
//     c();
// }

// fn c() {
//     panic!("hey i paniced in function c");
// }

// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }

use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;

fn main() {
    let mut file = match File::open("hello.rs") {
        Ok(file) => { file },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create("hello.rs") {
                        Ok(file) => { file },
                        Err(er) => { panic!("tried opening the hello.rs file. But, not found so, tried creating the same file but and error: {:#?} occured", er) }
                    }
                },
                _ => { panic!("problem in opening the file {:#?}",error) }
            }
        }
    };

    write!(file, r#"
    fn main() {{
        println!("hello world");
    }}
    "#).unwrap();
}