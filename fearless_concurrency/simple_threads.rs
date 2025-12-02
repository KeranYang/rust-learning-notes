// Using threads to run code simultaneously
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread, main doesn't wait for it to finish immediately.
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hi from the spawned thread! Count: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..3 {
        println!("Hi from the main thread! Count: {}", i);
        thread::sleep(Duration::from_millis(700));
    }

    // Wait for the spawned thread to finish using the join handle.
    // Calling join() **blocks** the current thread until the thread represented by the handle terminates.
    // join() is a blocking call.
    handle.join().unwrap();

    // Note: If we don't call join(), the main thread may finish before the spawned thread.
    // This would terminate the spawned thread prematurely.

    // ====================================================

    // Using move closures to transfer ownership to threads
    let data = vec![1, 2, 3, 4, 5];
    /* The problem statement:
    The following code won't work because the compiler doesn't know how long the spawned thread will live.
    If the main thread finishes and the data goes out of scope, the spawned thread would be left with a dangling reference.
    let handle2 = thread::spawn(|| {
        println!("Data in spawned thread: {:?}", data);
    });
    */
    /* The solution:
    We use the `move` keyword to transfer ownership of `data` into the closure.
    This way, the spawned thread owns `data`, and it will remain valid for the lifetime of the thread.
    */
    // The move keyword forces the closure to take ownership of **any** variables it uses from its environment.
    let handle2 = thread::spawn(move || {
        println!("Data in spawned thread: {:?}", data);
    });
    handle2.join().unwrap();
}