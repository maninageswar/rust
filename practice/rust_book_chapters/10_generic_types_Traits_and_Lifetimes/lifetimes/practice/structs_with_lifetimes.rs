// #[derive(Debug)]
// struct Preview<'a, 'b> {
//     title: &'a str,
//     body_preview: &'b str,
// }

// impl<'a, 'b> Preview<'a, 'b> {
//     fn new(title: &'a str, body: &'b str) -> Self {
//         Self {
//             title: title,
//             body_preview: body
//         }
//     }
// }

// fn make_preview(title: &str)-> Preview {
//     let body: String = String::from("make_previw function title")
//     Preview::new(title, &body)
// }

// fn main() {
//     let title = String::from("Rust Lifetimes");
//     let preview : Preview = make_preview(&title);
//     println!("the preview is {:#?}", preview);
// }

// fn main() {
//     let title = String::from("Rust Lifetimes");
//     let preview;
//     {
//         let body = String::from("This chapter is surprisingly important...");
//         preview = Preview::new(&title, &body);
//     }
//     println!("the preview is {:#?}", preview);
// }


#[derive(Debug)]
struct Preview<'a> {
    title: &'a str,
    body_preview: &'a str,
}

impl<'a> Preview<'a> {
    fn new(title: &'a str, body: &'a str) -> Self {
        Self {
            title: title,
            body_preview: body
        }
    }
}

fn main() {
    let title = String::from("Rust Lifetimes");
    
    {
        let body = String::from("Short lived body...");
        let preview = Preview::new(&title, &body);
    }
    
    // 3. Try using `title` here after the inner scope ends.
    println!("Title is still: {}", title);
}