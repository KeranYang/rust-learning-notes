// Note: rust's async operations are LAZY.

async fn print_world() {
    println!("world");
}

#[tokio::main]
/// The #[tokio::main] function is a macro.
/// It transforms the async fn main() into a synchronous fn main() that **initializes a runtime instance** and **executes the async main function**.
async fn main() {
    let operation = print_world(); // Create the future (lazy)
    println!("Hello,");
    operation.await; // Await the future to execute it
}

// Output:
// Hello,
// world
//
// In this example, we demonstrate that the async function is executed only when awaited.
// More details: the return value of an async function is a **Future**,
// which is a value representing the operation that will complete in the future.
//
// Any call to the **await** keyword will yield control back to the runtime,
// allowing other tasks to run while waiting for the awaited operation to complete.
// In this example, we have only one task, so the runtime simply resumes the task when the awaited operation is ready.
