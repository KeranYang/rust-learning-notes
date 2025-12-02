fn main() {
    shared_state_example();
}

fn shared_state_example() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Create a shared counter protected by a Mutex and wrapped in an Arc for thread-safe reference counting
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // calling Arc::clone to create a new reference to the same Arc
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex before accessing the shared counter
            let mut num = counter.lock().unwrap();
            *num += 1;
            // Mutex is automatically unlocked when `num` goes out of scope
        });
        handles.push(handle);
        // Here, the cloned Arc is dropped, but the original Arc remains valid.
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // lock the mutex to read the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap()); // Should print 10
    // Here, the original Arc is dropped, and since there are no more references,
    // the memory for the counter is freed.
}

// Notes:
// 1. Arc (Atomic Reference Counted)
// It is a smart pointer that enables **multiple ownership** of the same data **across different threads**.
// Its key features include 1. Thread-safe reference counting 2. Immutable data sharing 3. Cloneable references
// 2. Mutex (Mutual Exclusion)
// It's also a smart pointer that provides **interior mutability** with thread safety.
// It allows only one thread to access the data at a time, preventing data races.
// Key features include 1. Mutual exclusion 2. Interior mutability 3. Locking mechanism
// 3. Combining Arc and Mutex
// By combining Arc and Mutex, we can create a shared state that is both thread-safe and allows multiple ownership.
// This is particularly useful for scenarios where multiple threads need to read and write shared data.