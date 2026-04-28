struct Book {
    title: String,
    author: String,
}

fn print_book_title(book: &Book) {
    println!("the title of the book is {}", book.title);
}

fn main() {
    let book1: Book = Book {
        title: String::from("I want to work on what i like"),
        author: String::from("mani"),
    };
    print_book_title(&book1);
    println!("i am using the book fields after function call {}",book1.author);
}