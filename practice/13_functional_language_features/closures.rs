// fn main() {
//     let add_one = |x| x + 1;
//     println!("7 + 1 is equal to {}", add_one(7_u32));
//     println!("5 + 1 is equal to {}", add_one(5_u64));
// }

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    /* 
    Q : what happens when we do borrows_mutably(); will it call a method called 'call()' form the trait FnMut?

    A : Yes, you are exactly right conceptually! However, the specific method name inside the `FnMut` trait is **`call_mut`**, not `call`. 

    When you write `borrows_mutably();`, it is essentially "syntactic sugar". Behind the scenes, the Rust compiler translates that into a method call on the trait. 

    For your closure, it translates to something like this:
    ```rust
    FnMut::call_mut(&mut borrows_mutably, ());
    ```

    To give you the complete picture, Rust has three different closure traits, each with its own method:

    1. **`FnOnce`**: Uses **`call_once(self, args)`**
    * Takes ownership of the closure (`self`). Called when the closure consumes captured variables.
    2. **`FnMut`**: Uses **`call_mut(&mut self, args)`**
    * Mutably borrows the closure (`&mut self`). Called when the closure mutates captured variables (like your `list.push(7)`).
    3. **`Fn`**: Uses **`call(&self, args)`**
    * Immutably borrows the closure (`&self`). Called when the closure only reads captured variables.

    So yes, invoking the closure directly with the `()` syntax simply tells the compiler to route it to the `call_mut` method of the `FnMut` trait!
    */
    // println!("before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}