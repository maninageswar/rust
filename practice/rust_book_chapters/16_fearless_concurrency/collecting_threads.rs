use std::thread::{self, JoinHandle};

/// Sums each chunk on its own thread, keeping the input order.
fn sum_chunks(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    // TODO: spawn every thread first, collect the handles, then join
    let mut handles: Vec<JoinHandle<u64>> = vec![];
    for chunk in chunks.into_iter() {
        let handle: JoinHandle<u64> = thread::spawn(move || {
            chunk.iter().sum()
        });
        handles.push(handle);
    }
    handles.into_iter().map(|handle| handle.join().unwrap()).collect()
}

/// Sums every chunk in parallel and adds the partial sums together.
fn sum_all(chunks: Vec<Vec<u64>>) -> u64 {
    // TODO: reuse sum_chunks
    sum_chunks(chunks).into_iter().sum()
}

// Example usage
fn main() {
    let chunks = vec![vec![1, 2], vec![10], vec![]];

    println!("{:?}", sum_chunks(chunks.clone()));
    println!("{}", sum_all(chunks));
}
