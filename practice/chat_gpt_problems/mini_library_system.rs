// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a676f87-8cac-83e8-aeef-e3aaa41bae98

use std::time::Duration;

trait LibraryItem {
    fn get_id(&self) -> &u32;

    fn get_title(&self) -> &String;

    fn get_status(&self) -> &BorrowingStatus;

    fn can_be_borrowed(&self) -> bool;

    fn print_item(&self);

    fn borrow_item(&mut self, user: User, days: u32) -> Option<bool>;
}

#[derive(Debug)]
enum BorrowingStatus {
    Available,
    Borrowed {
        user: User,
        days: u32
    }
}

#[derive(Debug, Clone)]
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

    fn borrow_item(&mut self, user: User, days: u32) -> Option<bool> {
        if self.can_be_borrowed() {
            self.staus = BorrowingStatus::Borrowed {
                user,
                days
            };
            Some(true)
        } else {
            println!("sorry the item: {:#?} is not avaliable to borrow", self);
            None
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

    fn borrow_item(&mut self, user: User, days: u32) -> Option<bool> {
        if self.can_be_borrowed() {
            self.staus = BorrowingStatus::Borrowed {
                user,
                days
            };
            Some(true)
        } else {
            println!("sorry the item: {:#?} is not avaliable to borrow", self);
            None
        }
    }
}

#[derive(Debug)]
struct DVD {
    id: u32,
    title: String,
    duration_in_seconds: Duration,
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

    fn borrow_item(&mut self, user: User, days: u32) -> Option<bool> {
        if self.can_be_borrowed() {
            self.staus = BorrowingStatus::Borrowed {
                user,
                days
            };
            Some(true)
        } else {
            println!("sorry the item: {:#?} is not avaliable to borrow", self);
            None
        }
    }
}

#[derive(Debug)]
struct Library<T: LibraryItem> {
    name: String,
    items: Vec<T>
}

impl<T: LibraryItem> Library<T> {
    // the below associated fun and methods only avaliable for T where T implements LibraryItem trait
    // additionaly Library struct can store any T like it can also store items of Strings or u32 but those 
    // types does not make any sense right hence we defined trait bound which LibraryItem
    fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new()
        }
    }

    fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    fn borrow_library_item<'b>(&mut self, id: u32, user: &'b User, days: u32) -> Option<BorrowRecord<'_, 'b, T>> {
        /* please go through below points important
        
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
        learn why we should only use below but above line try using the above line see what error you will encounter

        iter_mut() Yields Mutable References: Calling .iter_mut() on a collection like a Vec<T> produces an iterator where each element is a mutable reference: &mut T.

        .find() Adds Another Reference Layer: The .find() method takes a closure and passes a reference to the item being evaluated. Because the item is already a &mut T, the closure receives item as &&mut T (a reference to a mutable reference).
        
        Automatic Dereferencing (Auto-Deref): When you use the dot operator (e.g., item.get_id()), Rust automatically dereferences the caller as many times as necessary. Even though item is &&mut T or &&&mut T, Rust will peel back those layers to find the base T that implements the method.
        
        Implicit Reference Coercion: If a method expects an immutable reference (&self / &T), but you pass it a mutable reference (&mut T inside the auto-deref process), Rust safely and automatically downgrades it to an immutable reference for that method call.
        
        Dereferencing the Result, Not the Caller: In the expression *item.get_id() == id, the * is not dereferencing item. Instead, it is dereferencing the return value of get_id() (which is &u32) into a plain u32 so it can be compared to the id parameter.
        
        Reference Matching (Your Way Works!): Instead of dereferencing the result, you can reference the target value. Doing item.get_id() == &id works perfectly because both sides are now &u32. Furthermore, adding extra reference layers deliberately (like let item_ref = &item;) still works because auto-dereferencing simply steps through as many pointers as necessary.
        */
        if let Some(item) = self.items.iter_mut().find(|item| {
            // here i have specifically created a ref of item(which is of type &&mut T) that makes it &&&mut T
            let item_ref = &item;
            // but the below line still works even though get_id() method only accepts &T that's because dereference coersion 
            item_ref.get_id() == &id
        }) {
            if let Some(borrowed_status) = item.borrow_item(user.clone(), days) {
                if borrowed_status == true {
                    // let borrow_record = BorrowRecord::new(item, &user, String::from("30-7-2026"));
                    // Or
                    let borrow_record: BorrowRecord<'_, 'b, T> = BorrowRecord::new(item, user, String::from("30-7-2026"));
                    return Some(borrow_record);
                }
                return None;
            }
            return None;
        } else {
            println!("item with id: {} not found in {}", id, self.name);
            return None;
        }
    }

    fn search_item(&self, id: u32) -> Option<&T> {
        self.items.iter().find(|item| item.get_id() == &id)
    }

    fn display_all_items(&self) {
        // for item in &self.items {
        //     item.print_item()
        // }
        // or
        self.items.iter().for_each(|item| item.print_item());
        // In the context of types, &T and &(T) are treated as exactly the same thing by the Rust compiler.
        // &value is exactly the same as &(value).
        // &self.items.iter().find(...) is exactly the same as &(self.items.iter().find(...))
    }

    fn display_borrowed_items(&self) {
        self.items.iter().filter(|item| item.can_be_borrowed() == false).for_each(|item| item.print_item());
    }

    fn display_avaliable_items(&self) {
        self.items.iter().filter(|item| item.can_be_borrowed() != false).for_each(|item| item.print_item());
    }
}


fn main() {
    let user1: User = User {
        name: String::from("sai"),
        email: String::from("sairam@gmail.com"),
        phone: 9876543210
    };

    let user2: User = User {
        name: String::from("shankar"),
        email: String::from("shankar@gmail.com"),
        phone: 9876543210
    };

    let mut book1: Book = Book::new(1, String::from("deep work"), String::from("cal newport"), 154, BorrowingStatus::Available);
    let mut book2: Book = Book::new(2, String::from("i can do it"), String::from("shankar"), 254, BorrowingStatus::Available);
    let mut book3: Book = Book::new(3, String::from("path to success"), String::from("manoj"), 354, BorrowingStatus::Available);
    // book1.print_item();
    // println!("the id of the book1 is {}", book1.get_id());
    // println!("the title of the book1 is {}", book1.get_title());
    // println!("the status of the book1 is {:?}", book1.get_status());
    // println!("can book1 be borrowed {}", book1.can_be_borrowed());
    // book1.borrow_item(user1, 7);
    // book1.print_item();
    let mut book_library1: Library<Book> = Library::new(String::from("vishal andhra"));
    book_library1.add_item(book1);
    book_library1.add_item(book2);
    book_library1.add_item(book3);
    println!("book_library1: {:#?}", book_library1);

    // if you want the below statement to work you have to remove the trait bound(LibraryItem) on Library struct (struct Library<T: LibraryItem> )
    // let mut book_library2: Library<String> = Library {
    //     name: String::from("vishal andhra"),
    //     items: vec![String::from("supreman1")],
    // };

    match book_library1.search_item(5) {
        Some(item) => println!("the details of the item that you are searching for is {:#?}", item),
        None => println!("sorry the item you are searching for does not exist in this library"),
    }

    println!("\ndisplay all items\n");
    book_library1.display_all_items();
    println!("\ndisplay all borrowed items\n");
    book_library1.display_borrowed_items();
    println!("\ndisplay all avaliable items\n");
    book_library1.display_avaliable_items();

    let mut magazine1: Magazine = Magazine {
        id: 1,
        title: String::from("my rules"),
        issue_number: 2343,
        publisher: String::from("general publishers"),
        staus: BorrowingStatus::Available,
    };

    let mut dvd1: DVD = DVD {
        id: 1,
        title: String::from("nezha1"),
        duration_in_seconds: Duration::from_secs((3 * 3600) + (2 * 60) + 7),
        genre: 23,
        staus: BorrowingStatus::Available,
    };

    print_any_library_item(&magazine1);
    // the trait bound `User: LibraryItem` is not satisfied
    // print_any_library_item(&user3);
    print_any_library_item(&dvd1);

    let mut dvd2: DVD = DVD {
        id: 2,
        title: String::from("nezha2"),
        duration_in_seconds: Duration::from_secs((3 * 3600) + (2 * 60) + 7),
        genre: 23,
        staus: BorrowingStatus::Available,
    };

    let user3: User = User {
        name: String::from("shankar"),
        email: String::from("shankar@gmail.com"),
        phone: 9876543210
    };

    let mut dvd_library1: Library<DVD> = Library::new(String::from("sony dvd's"));

    let mut dvd_borrow_history1: BorrowHistory<'_, '_, DVD> = BorrowHistory::new(String::from("dvd_borrow_history1"));
    // or
    // let mut dvd_borrow_history1 = BorrowHistory::<'_, '_, DVD>::new(String::from("dvd_borrow_history1"));

    println!("\nbefore borrowing");
    println!("\ndvd2: {:#?}",dvd2);
    println!("\ndvd_borrow_history1: {:#?}", dvd_borrow_history1);
    dvd_library1.add_item(dvd2);
    
    match dvd_library1.borrow_library_item(2, &user3, 6) {
        Some(borrowed_record) => dvd_borrow_history1.add_borrowed_record_to_borrow_history(borrowed_record),
        None => println!("sorry the item dvd2 cannot be borrowed for the library"),
    }

    println!("\nafter borrowing");
    println!("\ndvd_borrow_history1: {:#?}", dvd_borrow_history1);
}

fn print_any_library_item<T: LibraryItem>(item: &T) {
    item.print_item()
}

fn search_borrowed_record_form_borrow_history<'a, 'b, 'c, T: LibraryItem>(borrow_history_type: &'c BorrowHistory<'a, 'b, T>, item_id: u32) -> Option<&'c BorrowRecord<'a, 'b, T>>{
    borrow_history_type.borrow_history.iter().find(|item| item.borrowed_item.get_id() == &item_id)
}

#[derive(Debug)]
struct BorrowRecord<'a, 'b, T: LibraryItem> {
    borrowed_item: &'a T,
    borrowed_user: &'b User,
    borrow_date: String,
}

impl<'a, 'b, T: LibraryItem> BorrowRecord<'a, 'b, T> {
    fn new(borrowed_item: &'a T, borrowed_user: &'b User, borrow_date: String,) -> Self {
        Self {
            borrowed_item,
            borrowed_user,
            borrow_date,
        }
    }

    fn get_borrowed_item_user(&self) -> &User {
        self.borrowed_user
    }

    fn get_borrowed_item(&self) -> &T {
        self.borrowed_item
    }

    fn get_borrowed_date(&self) -> &String {
        &self.borrow_date
    }

    fn print_borrowed_item(&self) {
        println!("\nthe borrowed record is:");
        self.borrowed_item.print_item();
    }
}

#[derive(Debug)]
struct BorrowHistory<'a, 'b, T: LibraryItem> {
    name: String,
    borrow_history: Vec<BorrowRecord<'a, 'b, T>>,
}

impl <'a, 'b, T: LibraryItem> BorrowHistory<'a, 'b, T> {
    fn new(name: String) -> Self {
        Self {
            name,
            borrow_history: Vec::<BorrowRecord<'a, 'b, T>>::new(),
        }
    }

    fn add_borrowed_record_to_borrow_history(&mut self, borrowed_record: BorrowRecord<'a, 'b, T>) {
        self.borrow_history.push(borrowed_record);
    }
}