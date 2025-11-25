fn main() {
    // Note: The table below compares stack and heap memory in Rust.
    // | Aspect                 | Stack                                    | Heap                                     |
    // |------------------------|------------------------------------------|------------------------------------------|
    // | Memory Allocation      | Fixed size, allocated at compile time    | Dynamic size, allocated at runtime            |
    // | Access Speed           | Faster access due to contiguous memory    | Slower access due to pointer indirection    |
    // | Use Cases              | Primitive types, function call frames     | Complex data structures, large data       |
    // | Example Types          | Integers, floats, booleans, tuples       | Vectors, Strings, Box<T>                 |
    // | Memory Management      | Managed by Rust's ownership system          | Managed by Rust's ownership system, but requires heap allocation and deallocation |

    // With heap data, we need to ensure a proper garbage collection mechanism is in place to free up memory when it's no longer needed.
    // It means for every allocation on the heap, there must be a corresponding deallocation to prevent memory leaks.
    // Rust's ownership system helps manage this automatically through its borrowing and ownership rules.
    // Note: In Rust, the main purpose of the ownership system is to manage heap data safely and efficiently.

    // Note: i32 has a fixed size, it's stored on stack.
    let a = 10;
    let mut b = a;
    b += 5;
    // Note(the Copy trait): a and b are independent, because type i32 implements the Copy trait.
    // Stack-only data (e.g., integers, booleans, floats) implement the Copy trait by default.
    // This is because copy a fixed size of data is fast and efficient.
    println!("a = {a}, b = {b}"); // a = 10, b = 15

    // Note: String is a heap-allocated data structure, meaning the data (e.g., "Keran") is stored on the heap.
    let s = String::from("Keran");
    // Note: rust never auto-creates "deep copy" for heap data to avoid performance overhead.
    // Instead, it uses a move semantics to transfer ownership of heap data.
    let s2 = s; // the ownership of String is moved from s to s2, s is no longer valid
    // println!("s = {}", s); // -> compile error because the ownership of s is moved to s2
    println!("s2 = {s2}");

    // Note: to create a deep copy of heap data, we can use the clone() method.
    // this doesn't move the ownership, both s2 and s3 are valid and own their own heap data.
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // Note: rust will automatically free the heap memory when the variable goes out of scope.
    // here, an implicit drop(s2) and drop(s3) will be called at the end of this main function.

    // Note: In other words, heap deallocation principle - If a variable owns heap data,
    // when rust deallocates that variable's stack memory,
    // it also deallocates the heap memory associated with that variable.
}