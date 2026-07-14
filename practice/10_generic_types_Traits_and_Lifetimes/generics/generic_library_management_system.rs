// https://chatgpt.com/c/6a54e373-e870-83ee-ac52-94fcf643a4ba

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32
}

#[derive(Debug)]
struct Movie {
    title: String,
    director: String,
    duration: u32
}

#[derive(Debug)]
struct Magazine {
    title: String,
    issue: u32
}

#[derive(Debug)]
struct Resource<T> {
    id: u32,
    data: T
}

impl<T> Resource<T> {
    fn new(id: u32, data: T) -> Self {
        Self {
            id,
            data
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug)]
struct Library<T> {
    resources: Vec<Resource<T>>
}

impl<T> Library<T> {
    fn new() -> Self {
        Self {
            resources: Vec::new()
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
}

fn main() {
    let book1: Book = Book {
        title: String::from("my_great_life"),
        author: String::from("trumph"),
        pages: 4294967294
    };

    let movie1: Movie = Movie {
        title: String::from("my_dog"),
        director: String::from("crypto"),
        duration: 3600
    };

    let magazine1: Magazine = Magazine {
        title: String::from("super_man_success"),
        issue: 45
    };

    let resource1: Resource<Book> = Resource::new(1, book1);
    let resource2: Resource<Movie> = Resource::new(2, movie1);
    let resource3: Resource<Magazine> = Resource::new(3, magazine1);

    let mut generic_library: Library<Book> = Library {
        resources : Vec::new(),
    };

    generic_library.add_resource(resource1);
    println!("the generic library is {:#?}", generic_library);
    println!("the count of resources in generic library is {:#?}", generic_library.count());
    println!("is the generic library empty {:#?}", generic_library.is_enmpty());
    match generic_library.get_first_resource() {
        Some(resource) => {
            println!("the first resource of generic library {:#?}", resource);
        },
        None => { println!("the generic library is empty"); }
    }

    match generic_library.get_last_resource() {
        Some(resource) => {
            println!("the last resource of generic library {:#?}", resource);
        },
        None => { println!("the generic library is empty"); }
    }
}