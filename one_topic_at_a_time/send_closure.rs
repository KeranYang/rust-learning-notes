/// Recall that the `Send` marker trait ensures that a type can be safely transferred across thread boundaries.
/// It ensures that when we switch between tasks, the intermediate state of the future can be safely persisted in a storage, such that later, when a new task is spawned, the state can be restored. More details in https://tokio.rs/tokio/tutorial/spawning#send-bound.
///
/// Now, it's quite straightforward that a primitive type like i32 is Send, simply because it can be copied.
///
/// However, what about a closure?
///
/// It's common in multi-threaded programming to send a closure across threads. What does it mean for a closure to be Send?
///
/// A closure is Send if all the variables it captures are Send. This is because the closure needs to be able to be safely transferred across threads, and if it captures any non-Send variables, it would not be safe to do so.
fn send_closure() {
    let x = 5;
    let y = String::from("hello");
    let z = Box::new(10);

    // This particular closure is Send because the captured variables x, y, and z are all Send.
    let working_closure = move || {
        println!("x: {}", x);
        println!("y: {}", y);
        println!("z: {}", z);
    };

    std::thread::spawn(working_closure).join().unwrap();
}

fn main() {
    send_closure();
}
