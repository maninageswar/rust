use std::thread::{self, ScopedJoinHandle};

/// Finds the largest value by scanning both halves in parallel.
pub fn parallel_max(data: &[i32]) -> Option<i32> {
    // TODO: split `data` in half and compare both halves inside a
    // thread::scope, without cloning or using Arc
    let result = thread::scope(|s| {
        let handle1 = s.spawn(|| {
            data[..data.len()/2].iter().max().copied()
        });
        let handle2 = s.spawn(|| {
            data[data.len()/2..].iter().max().copied()
        });
        let value1: i32 = handle1.join().unwrap().unwrap();
        let value2: i32 = handle2.join().unwrap().unwrap();
        if value1 > value2 {
            return Some(value1);
        } else {
            return Some(value2);
        }
        return None;
    });
    result
}

/// Splits values into evens and odds using two scoped threads.
pub fn split_evens_odds(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut evens = Vec::<i32>::new();
    let mut odds = Vec::<i32>::new();

    // TODO: fill both vectors from inside a thread::scope
    thread::scope(|scope| {
        let handle1: ScopedJoinHandle<'_, Vec<i32>> = scope.spawn(|| { data.iter().filter(|num| *num % 2 == 0).copied().collect() });
        let handle2: ScopedJoinHandle<'_, Vec<i32>> = scope.spawn(|| { data.iter().filter(|num| *num % 2 != 0).copied().collect() });
        // the below two lines won't push anything to both evens and odds cuz map is lazy as it does not have consuming adapter
        // handle1.join().unwrap().into_iter().map(|num| evens.push(num));
        // handle2.join().unwrap().into_iter().map(|num| odds.push(num));
        for num in handle1.join().unwrap().into_iter() {
            evens.push(num)
        }
        for num in handle2.join().unwrap().into_iter() {
            odds.push(num)
        }
    });

    (evens, odds)
}

// Example usage
pub fn main() {
    println!("{:?}", parallel_max(&[7]));
    println!("{:?}", split_evens_odds(&[1, 2, 3, 4]));
}
