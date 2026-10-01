use std::thread::{self, JoinHandle};

/// Sums a list of numbers on a separate thread and returns the total.
fn sum_in_thread(values: Vec<u64>) -> u64 {
    // TODO: spawn a thread that sums `values`, then join it
    let handle: JoinHandle<u64> = thread::spawn(move || values.iter().sum());
    handle.join().unwrap()
}

/// Starts a thread that counts and returns the handle immediately.
///
/// The thread adds up `start + (start + 1) + ... + (start + steps - 1)`.
fn spawn_counter(start: u64, steps: u64) -> JoinHandle<u64> {
    // TODO: spawn the thread and return its handle without joining
    let handle: JoinHandle<u64> = thread::spawn(move || {
        let mut total: u64 = 0;
        for i in 0..steps {
            total += start + i;
        }
        return total;
    });
    handle
}

// Example usage
fn main() {
    println!("sum: {}", sum_in_thread(vec![1, 2, 3, 4]));
    println!("spawning counter thread...");
    let handle = spawn_counter(10, 3);
    println!("counter: {}", handle.join().unwrap());
}