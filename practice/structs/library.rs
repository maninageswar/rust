#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_avaliable: bool,
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn add_a_book(self: &mut Self, book: Book) {
        // self is &mut Library — a mutable reference to the Library instance.
        // self.books is not a reference; it's the actual Vec<Book> field accessed
        // through self. Because self is &mut, we have mutable access to all fields.
        // .push() needs &mut Vec<Book>, so Rust auto-borrows self.books as
        // &mut self.books behind the scenes.
        self.books.push(book);
    }

    fn borrow_a_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                book.is_avaliable = false;
                break;
            }
        }
    }

    fn is_book_avaliable(&self, title: &str) -> Option<bool> {
        for book in &self.books {
            if book.title == title {
                return Some(book.is_avaliable);
            }
        }
        None
    }

    fn return_a_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                book.is_avaliable = true;
            }
        }
    }

    fn list_of_avaliable_of_books(&self) -> Vec<&Book> {
        let mut avaliable_books_list = Vec::<&Book>::new();
        for book in &self.books {
            if book.is_avaliable {
                avaliable_books_list.push(book);
            }
        }
        avaliable_books_list
    }

    fn find_book(&self, title: &str) -> Option<&Book> {
        for book in &self.books {
            if book.title == title {
                return Some(&book)
            }
        }
        None
    }
}

fn main() {

    let book1 = Book {
        title: String::from("title1"),
        author: String::from("author1"),
        is_avaliable: true,
    };

    let book2 = Book {
        title: String::from("title2"),
        author: String::from("author2"),
        is_avaliable: true,
    };

    let book3 = Book {
        title: String::from("title3"),
        author: String::from("author3"),
        is_avaliable: false,
    };

    let book4 = Book {
        title: String::from("title4"),
        author: String::from("author4"),
        is_avaliable: true,
    };

    let book5 = Book {
        title: String::from("title5"),
        author: String::from("author5"),
        is_avaliable: true,
    };

    let mut library1 = Library {
        books: Vec::<Book>::new(),
    };

    library1.add_a_book(book1);
    library1.add_a_book(book2);
    library1.add_a_book(book3);
    library1.add_a_book(book4);
    library1.add_a_book(book5);

    println!("title1 borrowed"); // title1 borrowed
    library1.borrow_a_book("title1");
    check_if_book_is_avaliable(&library1, "title1");

    println!();

    println!("atitle1 returned"); // title1 returned
    library1.return_a_book("title1");
    check_if_book_is_avaliable(&library1, "title1");

    println!("title5 borrowed"); // title5 borrowed
    library1.borrow_a_book("title5");

    println!("list of avaliable books are {:#?}", library1.list_of_avaliable_of_books());
    
    // find book4
    find_book_in_library(&library1, "title4");
}

fn find_book_in_library(library: &Library, title: &str) {
    match library.find_book(title) {
        Some(book) => {
            println!("book with title: {} is present in library1 here are it's details: {:#?}", title, book);
        },
        None => { println!("book with title: {} is not present in library1",title) },
    }
}

fn check_if_book_is_avaliable(library: &Library,title: &str) {
    match library.is_book_avaliable(title) {
        Some(status) => { 
            if status == true {
                println!("library1 has the book which has the title: title1");
            } else {
                println!("library1 does not have the book which has the title: title1");
            }
        },
        None => { println!("the book with the title: title1 does not exist in the library1") },
    }
}