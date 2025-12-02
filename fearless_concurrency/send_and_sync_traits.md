### What is a marker trait in Rust?

A marker trait in Rust is a special type of trait that does not contain any methods or associated items.
Instead, it is used to convey metadata about a type, indicating that the type has certain properties or behaviors. 
Marker traits are often used for compile-time checks and to enforce certain constraints on types.

### The Send marker trait

The `Send` marker trait indicates that a type is safe to be transferred across thread boundaries.
If a type implements the `Send` trait,
it means that instances of that type can be sent from one thread to another without causing data races.

Some key points about the `Send` trait:
- Most primitive types in Rust (like integers, floats, and booleans) implement the `Send` trait.
- Types that contain non-`Send` types (like raw pointers or types that manage their own memory) do not implement the `Send` trait by default.
- The Rust compiler automatically implements the `Send` trait for types that are composed entirely of `Send` types.

Smart Pointers that do not implement `Send` by default include:
- `Rc<T>`: Reference-counted smart pointer for single-threaded scenarios.

Smart pointers that implement `Send` include:
- `Arc<T>`: Atomic reference-counted smart pointer for multithreaded scenarios.

### The Sync marker trait

The `Sync` marker trait indicates that a type is safe to be referenced from multiple threads **simultaneously**.
If a type implements the `Sync` trait,
it means that references to instances of that type can be shared across threads without causing data races.

Some key points about the `Sync` trait:
- Most primitive types in Rust implement the `Sync` trait.
- Types that contain non-`Sync` types do not implement the `Sync` trait by default.
- The Rust compiler automatically implements the `Sync` trait for types that are composed entirely of `Sync` types.

Smart Pointers that do not implement `Sync` by default include:
- `Rc<T>`: Reference-counted smart pointer for single-threaded scenarios.

Smart pointers that implement `Sync` include:
- `Arc<T>`: Atomic reference-counted smart pointer for multithreaded scenarios.

### Comparison of Send and Sync

Types that implement `Send` but not `Sync` include `Mutex<T>`, which allows safe transfer of ownership between threads but does not allow simultaneous access from multiple threads.

Types that implement `Sync` but not `Send` are rare, but an example could be a type that contains a reference to a non-`Send` type, allowing shared access but not ownership transfer.