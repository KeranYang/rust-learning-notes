# Rust comparison with other languages

## Why is Rust better than JavaScript

Strong Type System/Strict Compiler to ensure that types are checked at compile time, preventing runtime errors.

JavaScript:

```javascript
function add(a, b) {
  return a + b;
}

const result = add(2, "3");
console.log(result); // Output: 23 -> LOGICAL ERROR, very bad! -> pass compilation and give unexpected result at runtime, dangerous!
```

Rust:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let result = add(2, 3);
// let result = add(2, "3"); // Compile time error: mismatched types
println!("Result: {}", result); // Output: Result: 5
```

## Why is Rust better than Python

Again, Strong Type System/Strict Compiler to ensure that types are checked at compile time, preventing runtime errors.

Python:

```python
def get_length(s) {
    return len(s)

println(get_length(10)) // This will pass compilation and give runtime error: TypeError: object of type 'int' has no len(), dangerous!
```

## Why is Rust better than C and C++

Effecient memmory management.

In C and C++, we have `malloc` and `free`/`delete` to allocate and deallocate memory. With such superpower, programmers can easily make mistakes and cause memory leaks or buffer overflows.
Different kinds of mistakes can be made:

- Double freeing corrupting memory
- freeing too early causing invalid memory access
- freeing too late wasting memory
- forgetting to free causing memory leaks
- etc.

Manually managing memory is a bad idea.

In rust, we no longer give the programmer such superpower. Rust completely takes care of memory management for us. It always puts the drop in the right place, during compilation.

```rust
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("Vector: {:?}", vec); // Output: Vector: [1, 2, 3]
    // a drop(vec) is added here by the compiler, ensuring memory is freed immediately after vec goes out of scope.
}
```

## Why is Rust better than Java, C++, GoLang and JavaScript

Garbage collection is a bad idea.

Garbage collection is a low-priority thread that periodically runs in the background to find and free unused memory. This can lead to **performance** issues. It takes CPU cycles and affects the main thread's performance.

Rust's ownership system ensures allocated memory is freed automatically the moment its owner variable goes out of scope.
