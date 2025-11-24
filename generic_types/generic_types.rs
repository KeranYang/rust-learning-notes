// Note: I think generic types are observed as opposed to explicitly taught.
// In other words,
// You recognize situations where generics can be applied to make code more flexible and reusable.
// You then modify function signatures and implementations to use generic type parameters instead of concrete types.
fn main() {
    // Note: performance-wise, generics in Rust are implemented using monomorphization,
    // which means that the compiler generates specialized versions of generic functions
    // and types for each concrete type they are used with at compile time. Hence, there is no runtime overhead for using generics in Rust.

    // Questions about generic types:
    /*
Question: Does the following code compile? If it does, what is the expected output when executed?

```
fn print_slice<T>(v: &[T]) {
  for x in v {
    println!("{x}");
  }
}
fn main() {
  print_slice(&[1, 2, 3]);
}
```

Answer: No, the code does not compile.
Since T is a generic type, we cannot assume that it implements the Display trait required for the println! macro.
     */
}

// Note: T: PartialOrd means that the type T must implement the PartialOrd trait,
// which allows for comparison operations like greater than (>) and less than (<).
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}