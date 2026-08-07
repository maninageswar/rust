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

    fn delete_user(&mut self, user_details: &User) {
        self.users.retain(|user| user.name != user_details.name);
        /* {
            User: error[E0507]: cannot move out of a shared reference
            --> auth_system.rs:42:28
            |
            42 |         self.users.retain(|&user| user.name != user_details.name);
            |                            ^----
            |                             |
            |                             data moved here
            |                             move occurs because `user` has type `User`, which does not implement the `Copy` trait
            |
            help: consider removing the borrow
            |
            42 -         self.users.retain(|&user| user.name != user_details.name);
            42 +         self.users.retain(|user| user.name != user_details.name);

            can you explain this error

            GitHub Copilot: The issue is on line 42 in the `retain` closure:

            ```rust
            self.users.retain(|&user| user.name != user_details.name);
            ```

            `retain` passes each element as `&User` (a shared reference). When you write `|&user|`, you're using **pattern matching to destructure** the reference — this attempts to move the `User` value out of the reference into `user`. Since `User` contains `String` fields (which don't implement `Copy`), Rust can't move the value out of a shared reference.

            The fix is to use `|user|` instead:

            ```rust
            self.users.retain(|user| user.name != user_details.name);
            ```

            Here, `user` has type `&User` — you're just binding the reference directly, no move occurs. Accessing `user.name` still works because Rust auto-dereferences.

            **In short:**
            - `|&user|` = "destructure the reference, give me the owned value" → requires a move → fails
            - `|user|` = "give me the reference as-is" → no move → works fine

            User: what exctaly does retain do will it pass each element in users as a reference?

            GitHub Copilot: Yes, exactly. `Vec::retain` iterates over every element in the vector and passes each one **as a shared reference (`&T`)** to the closure. The closure must return a `bool`:

            - Return `true` → the element is **kept**
            - Return `false` → the element is **removed**

            So for your code:

            ```rust
            self.users.retain(|user| user.name != user_details.name);
            //                 ^^^^
            //                 user: &User
            ```

            `retain` goes through each `User` in `self.users`, passes it as `&User` to the closure, and removes any where `user.name == user_details.name`.

            The signature of `retain` is:

            ```rust
            pub fn retain<F>(&mut self, f: F)
            where
                F: FnMut(&T) -> bool,
            ```

            Key points:
            - The **vector** is borrowed mutably (`&mut self`) because elements may be removed.
            - Each **element** is passed as `&T` (shared reference) to the closure — you can inspect it but not move out of it or mutate it.
            - It's essentially a filter-in-place: equivalent to keeping only elements that satisfy the predicate, without allocating a new vector.

            User: self.users.retain(|user| user.name != user_details.name);

            here user is a reference what happens if i use |&user| insted |user| what i think is we will have reference of reference like |&(&User)| is that a problem why is that a problem

            GitHub Copilot: Your mental model is slightly off — it's actually the **opposite** of what you think.

            `|&user|` is not adding another layer of reference. It's **pattern matching** that tries to **strip away** the reference.

            Here's what happens:

            ### `|user|` — bind the reference as-is
            ```rust
            // retain passes: &User
            // user binds to: &User  ✅
            |user| user.name != user_details.name
            //  user: &User — just holding a reference, no move
            ```

            ### `|&user|` — destructure/strip the reference
            ```rust
            // retain passes: &User
            // pattern &user matches against &User
            // so user binds to: User (the owned value!)  ❌
            |&user| user.name != user_details.name
            //  user: User — tries to move the User out of the reference
            ```

            The `&` in `|&user|` is a **destructuring pattern**, like in `let &x = some_reference;`. It peels off the `&`, attempting to give you the **owned inner value**. This requires **moving** the `User` out from behind the reference.

            That's illegal because:
            1. `retain` only gives you a **shared reference** (`&User`) — you can look but not take ownership
            2. `User` contains `String` fields which don't implement `Copy`, so it can't be implicitly copied

            If `User` only contained `Copy` types (like `i32`, `bool`), then `|&user|` would work fine — Rust would just copy the value out.

            **Summary:**
            | Pattern | `user` type | What happens |
            |---------|------------|--------------|
            | `\|user\|` | `&User` | Binds the reference — works |
            | `\|&user\|` | `User` | Tries to move out of reference — fails |
        } */
    }
}

#[derive(Debug)]
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
    show_auth_status(user1_sign_up_status);

    println!();

    let user1_login_status: AuthStatus = authenticator.login(&User {
        name: String::from("sairam"),
        password: String::from("123456789"),
    });
    show_auth_status(user1_login_status);

    println!();

    authenticator.sign_up(user2);

    let user3_sign_up_status: AuthStatus = authenticator.sign_up(user3);
    show_auth_status(user3_sign_up_status);

    println!();

    let user3_login_status: AuthStatus = authenticator.login(&User {
        name: String::from("hemanth"),
        password: String::from("123456789"),
    });
    show_auth_status(user3_login_status);

    println!("user databese before deleting user3 is:");
    println!("{:#?}",authenticator.authdb);

    authenticator.authdb.delete_user(&User {
        name: String::from("hemanth"),
        password: String::from("123456789"),
    });

    println!("user databese after deleting user3 is:");
    println!("{:#?}",authenticator.authdb);
}

fn show_auth_status(auth_status: AuthStatus) {
    match auth_status {
        AuthStatus::Success(message) => {
            println!("{message}")
        }
        AuthStatus::Failure(message) => {
            println!("{message}")
        }
    }
}
