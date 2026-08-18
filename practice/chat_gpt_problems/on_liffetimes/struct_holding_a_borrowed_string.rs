struct User<'a> {
    name: &'a str,
}

fn create_user() -> User<'_> {
    let name: String = String::from("raju");
    User {
        name: &name,
    }
}

fn main() {
    let user1: User = create_user();
}