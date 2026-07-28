// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a676f87-8cac-83e8-aeef-e3aaa41bae98

trait LibraryItem {
    fn get_id(&self) -> &u32;

    fn get_title(&self) -> &String;

    fn get_status(&self) -> &BorrowingStatus;

    fn can_be_borrowed(&self) -> bool;

    fn print_item(&self);

    fn borrow_item(&mut self, user: User, days: u32);
}

#[derive(Debug)]
enum BorrowingStatus {
    Available,
    Borrowed {
        user: User,
        days: u32
    }
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    phone: u64
}

#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    no_of_pages: u32,
    staus: BorrowingStatus
}

impl Book {
    fn new(id: u32, title: String, author: String, no_of_pages: u32, staus: BorrowingStatus) -> Self {
        Self {
            id,
            title,
            author,
            no_of_pages,
            staus
        }
    }
}

impl LibraryItem for Book {
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn get_status(&self) -> &BorrowingStatus {
        &self.staus
    }

    fn can_be_borrowed(&self) -> bool {
        if let BorrowingStatus::Available = self.get_status() {
            return true;
        } else {
            return false;
        }
    }

    fn print_item(&self) {
        println!("{:#?}", self);
    }

    fn borrow_item(&mut self, user: User, days: u32) {
        if self.can_be_borrowed() {
            self.staus = BorrowingStatus::Borrowed {
                user,
                days
            }
        } else {
            println!("sorry the item: {:?} is not avaliable to borrow", self)
        }
    }
}

#[derive(Debug)]
struct Magazine {
    id: u32,
    title: String,
    issue_number: u32,
    publisher: String,
    staus: BorrowingStatus
}

impl LibraryItem for Magazine {
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn get_status(&self) -> &BorrowingStatus {
        &self.staus
    }

    fn can_be_borrowed(&self) -> bool {
        if let BorrowingStatus::Available = self.get_status() {
            return true;
        } else {
            return false;
        }
    }

    fn print_item(&self) {
        println!("{:#?}", self);
    }

    fn borrow_item(&mut self, user: User, days: u32) {
        if self.can_be_borrowed() {
            self.staus = BorrowingStatus::Borrowed {
                user,
                days
            }
        } else {
            println!("sorry the item: {:?} is not avaliable to borrow", self)
        }
    }
}

#[derive(Debug)]
struct DVD {
    id: u32,
    title: String,
    duration_in_minutes: String,
    genre: u32,
    staus: BorrowingStatus
}

impl LibraryItem for DVD {
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn get_status(&self) -> &BorrowingStatus {
        &self.staus
    }

    fn can_be_borrowed(&self) -> bool {
        if let BorrowingStatus::Available = self.get_status() {
            return true;
        } else {
            return false;
        }
    }

    fn print_item(&self) {
        println!("{:#?}", self);
    }

    fn borrow_item(&mut self, user: User, days: u32) {
        if self.can_be_borrowed() {
            self.staus = BorrowingStatus::Borrowed {
                user,
                days
            }
        } else {
            println!("sorry the item: {:?} is not avaliable to borrow", self)
        }
    }
}

struct Library<T: LibraryItem> {
    name: String,
    items: Vec<T>
}

impl<T: LibraryItem> Library<T> {
    fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new()
        }
    }

    fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    fn borrow_library_item(&mut self, id: u32, user: User, days: u32) {
        // if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
        // learn why we should only use below but above line try using the above line see what error you will encounter
        if let Some(item) = self.items.iter_mut().find(|item| *item.get_id() == id) {
            item.borrow_item(user, days);
        } else {
            println!("item with id: {} not found in {}", id, self.name)
        }
    }
}

fn main() {
    let user1: User = User {
        name: String::from("sai"),
        email: String::from("sairam@gmail.com"),
        phone: 9876543210
    };
    let mut book1: Book = Book::new(1, String::from("deep work"), String::from("cal newport"), 154, BorrowingStatus::Available);
    book1.print_item();
    println!("the id of the book1 is {}", book1.get_id());
    println!("the title of the book1 is {}", book1.get_title());
    println!("the status of the book1 is {:?}", book1.get_status());
    println!("can book1 be borrowed {}", book1.can_be_borrowed());
    book1.borrow_item(user1, 7);
    book1.print_item();
}