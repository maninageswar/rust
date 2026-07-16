//see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a54e373-e870-83ee-ac52-94fcf643a4ba

trait DisplayInfo {
    fn display_info(&self) -> String;
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl DisplayInfo for Book {
    fn display_info(&self) -> String {
        format!(
            "The {} is written by {} which has {} pages",
            self.title, self.author, self.pages
        )
    }
}

#[derive(Debug, Clone)]
struct Movie {
    title: String,
    director: String,
    duration: u32,
}

impl DisplayInfo for Movie {
    fn display_info(&self) -> String {
        format!(
            "The {} is directed by {} which has duration of {}",
            self.title, self.director, self.duration
        )
    }
}

#[derive(Debug)]
struct Magazine {
    title: String,
    issue: u32,
}

#[derive(Debug)]
struct Resource<T> {
    id: u32,
    data: T,
}

impl<T> Resource<T> {
    fn new(id: u32, data: T) -> Self {
        Self { id, data }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug)]
struct Library<T: DisplayInfo> {
    resources: Vec<Resource<T>>,
}

impl<T: DisplayInfo> Library<T> {
    fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    fn add_resource(&mut self, resource: Resource<T>) {
        self.resources.push(resource);
    }

    fn count(&self) -> usize {
        self.resources.len()
    }

    fn is_enmpty(&self) -> bool {
        self.resources.len() > 1
    }

    fn get_first_resource(&self) -> Option<&Resource<T>> {
        self.resources.get(0)
    }

    fn get_last_resource(&self) -> Option<&Resource<T>> {
        self.resources.get(self.count() - 1)
    }

    fn remove_resource_by_id(&mut self, id: u32) {
        // error[E0502]: cannot borrow `self.resources` as mutable because it is also borrowed as immutable
        // for the below commented code you will get the above error
        // for (index, resource) in self.resources.iter().enumerate() {
        //     if resource.id == id {
        //         self.resources.remove(index);
        //     }
        // }
        self.resources.retain(|resource| resource.id != id);
    }

    fn print_all(&self) {
        // for resource in &self.resources {
        // or
        for resource in self.resources.iter() {
            println!("{}",resource.get_data().display_info());
        }
    }
}

#[derive(Debug)]
enum Gender {
    Male,
    Female
}

#[derive(Debug)]
struct User {
    name: String,
    gender: Gender,
    age: u8
}

#[derive(Debug)]
struct BorrowRecord<U, T> {
    user: U,
    resource: Resource<T>
}

impl<U, T>  BorrowRecord<U, T> {
    fn new(user: U, resource: Resource<T>) -> Self {
        Self {
            user,
            resource
        }
    }

    fn get_user(&self) -> &U {
        &self.user
    }

    fn get_resource(&self) -> &Resource<T> {
        &self.resource
    }
}

fn main() {
    let book1: Book = Book {
        title: String::from("my_great_life"),
        author: String::from("trumph"),
        pages: 4294967294,
    };

    let book2: Book = Book {
        title: String::from("na_chavu_nenu_chasta_nekenduku"),
        author: String::from("koushik"),
        pages: 4294967294,
    };

    let movie1: Movie = Movie {
        title: String::from("my_dog"),
        director: String::from("crypto"),
        duration: 3600,
    };

    let magazine1: Magazine = Magazine {
        title: String::from("super_man_success"),
        issue: 45,
    };

    let resource1: Resource<Book> = Resource::new(1, book1);
    let resource2: Resource<Movie> = Resource::new(2, movie1);
    let resource3: Resource<Magazine> = Resource::new(3, magazine1);
    let resource4: Resource<Book> = Resource::new(4, book2);

    let mut generic_library1: Library<Book> = Library {
        resources: Vec::new(),
    };

    // the trait `DisplayInfo` is not implemented for `Magazine`
    // you will get the above error for the below code
    // let mut generic_library2: Library<Magazine> = Library {
    //     resources: Vec::new(),
    // };

    generic_library1.add_resource(resource1);
    generic_library1.add_resource(resource4);
    println!("the generic library is {:#?}", generic_library1);

    println!("the count of resources in generic library is {:#?}", generic_library1.count());
    println!("is the generic library empty {:#?}", generic_library1.is_enmpty());
    match generic_library1.get_first_resource() {
        Some(resource) => {
            println!("the first resource of generic library {:#?}", resource);
        },
        None => { println!("the generic library is empty"); }
    }

    match generic_library1.get_last_resource() {
        Some(resource) => {
            println!("the last resource of generic library {:#?}", resource);
        },
        None => { println!("the generic library is empty"); }
    }

    generic_library1.remove_resource_by_id(4);
    println!("the generic library after removing the resource with id 4 is {:#?}", generic_library1);

    generic_library1.print_all();

    let user1: User = User {
        name: String::from("vasu"),
        gender: Gender::Male,
        age: 25
    };

    let user2: String = String::from("ravi");

    let book3: Book = Book {
        title: String::from("nenu_eppudu_mani_gadi_call_lift_cheyyanu"),
        author: String::from("tarun"),
        pages: 4294967294,
    };

    let resource5: Resource<Book> = Resource::new(4, book3);


    let borrow1 = BorrowRecord::<String, Book>::new(user2, resource5);
    println!("the borrow1 is {:#?}", borrow1);
    println!("the user of the borrow1 is {}", borrow1.get_user());
    println!("the Resource of the borrow1 is {:#?}", borrow1.get_resource());

    let book4: Book = Book {
        title: String::from("i_am_the_king"),
        author: String::from("vasu"),
        pages: 4294967294,
    };

    let resource6: Resource<Book> = Resource::new(4, book4);

    // let borrow2: BorrowRecord<User, Book> = BorrowRecord::<User, Book>::new(user1, resource6);
    // or
    // let borrow2: BorrowRecord<User, Book> = BorrowRecord::new(user1, resource6);
    // or
    // let borrow2 = BorrowRecord::<User, Book>::new(user1, resource6);
    // or
    let borrow2 = BorrowRecord::new(user1, resource6);
    println!("the borrow1 is {:#?}", borrow2);
    println!("the user of the borrow1 is {:#?}", borrow2.get_user());
    println!("the Resource of the borrow2 is {:#?}", borrow2.get_resource());

    let book5: Book = Book {
        title: String::from("i_am_the_superman"),
        author: String::from("sai"),
        pages: 4294967294,
    };

    let book6: Book = Book {
        title: String::from("my_travel"),
        author: String::from("kumar"),
        pages: 4294967294,
    };

    println!();
    let mut resource7: Resource<Book> = Resource::new(5, book5);
    let mut resource8: Resource<Book> = Resource::new(6, book6);
    println!("the resource7  before swap is {:#?}", resource7);
    println!("the resource8  before swap is {:#?}", resource8);
    swap_same_resources(&mut resource7, &mut resource8);
    println!();
    println!("the resource7  after swap is {:#?}", resource7);
    println!("the resource8  after swap is {:#?}", resource8);

    let book7: Book = Book {
        title: String::from("i_like_food"),
        author: String::from("shankar"),
        pages: 4294967294,
    };

    let movie2: Movie = Movie {
        title: String::from("ready_player_one"),
        director: String::from("mike"),
        duration: 3600,
    };
}

fn swap_same_resources<T: Clone>(r1: &mut Resource<T>, r2: &mut Resource<T>) {
    let temp: T = r1.data.clone();
    r1.data = r2.data.clone();
    r2.data = temp;
}