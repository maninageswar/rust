enum AuthStatus {
    Success(String),
    Failure(String),
}

#[derive(Debug)]
struct User {
    name: String,
    password: String,
}

impl User {
    fn is_password_correct(&self, user: &User) -> bool {
        if self.password == user.password {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct UsersDatabase {
    users: Vec<User>,
}

impl UsersDatabase {
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn find_user(&self, user_name: &str) -> Option<&User> {
        for user in &self.users {
            if user.name == user_name {
                return Some(user);
            }
        }
        None
    }
}

struct Authenticate {
    authdb: UsersDatabase,
}

impl Authenticate {
    fn sign_up(&mut self, user_details: User) -> AuthStatus {
        match &self.authdb.find_user(&user_details.name) {
            Some(user_refrence) => AuthStatus::Failure(String::from(
                "there is already a user with the same. please try with another user name",
            )),
            None => {
                &self.authdb.add_user(user_details);
                AuthStatus::Success(String::from("you have successfully signed in"))
            }
        }
    }

    fn login(&self, user_details: &User) -> AuthStatus {
        match &self.authdb.find_user(&user_details.name) {
            Some(user_reference) => {
                if user_reference.is_password_correct(user_details) {
                    AuthStatus::Success(String::from("you have successfully logged in"))
                } else {
                    AuthStatus::Success(String::from("incorrect password"))
                }
            }
            None => AuthStatus::Failure(String::from(
                "we could not find your details in our system please sign up to login",
            )),
        }
    }
}

fn main() {
    let userDatabase = UsersDatabase {
        users: Vec::<User>::new(),
    };

    let mut authenticator = Authenticate {
        authdb: userDatabase,
    };

    let user1 = User {
        name: String::from("sairam"),
        password: String::from("1234567890"),
    };

    let user2 = User {
        name: String::from("saishankar"),
        password: String::from("1234567890"),
    };

    let user3 = User {
        name: String::from("hemanth"),
        password: String::from("1234567890"),
    };

    let user1_sign_up_status: AuthStatus = authenticator.sign_up(user1);
    match user1_sign_up_status {
        AuthStatus::Success(message) => {
            println!("{message}")
        }
        AuthStatus::Failure(message) => {
            println!("{message}")
        }
    }

    println!();

    let user1_login_status: AuthStatus = authenticator.login(&User {
        name: String::from("sairam"),
        password: String::from("123456789"),
    });
    match user1_login_status {
        AuthStatus::Success(message) => {
            println!("{message}")
        }
        AuthStatus::Failure(message) => {
            println!("{message}")
        }
    }
}

