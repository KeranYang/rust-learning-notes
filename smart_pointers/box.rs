// what is a Box in rust?
// A Box<T> is a smart pointer in Rust that allocates data on the heap.
// It provides ownership for heap-allocated data and ensures that the data is properly deallocated when the Box goes out of scope.
fn main() {
    simple_box_example();
}

fn simple_box_example() {
    let b1 = Box::new(20);
    let b2 = b1; // Ownership of the Box is moved to b2
    // println!("b1: {}", b1); // -> compile error because b1 is no longer valid
    println!("b2: {}", b2); // b2 is valid

    // Why is the ownership moved?
    // Note: data in Box is heap allocated and does not implement the Copy trait.
    // rust enforces ownership rules on heap data to ensure memory safety.

    // What if rust doesn't throw an error when we use b1 after the move?
    // What undefined behavior could happen?
    //
    // Two types of problems could occur:
    // 1. Double Free Error if both b1 and b2 were allowed to own the same Box.
    // 2. Use After Free Error if b1 was used after b2 has been dropped.

    // Note: heap deallocation principle - If a variable owns heap data,
    // when rust deallocates that variable's stack memory,
    // it also deallocates the heap memory associated with that variable.
}