# Comparison of `const` and `static` in Rust

In Rust, both `const` and `static` are used to define values that are constant and immutable, but they have some key differences in terms of their usage, memory allocation, and lifetime.

## `const`

- `const` defines a constant value that is inlined wherever it is used.
- It must be initialized with a constant expression, meaning the value must be known at compile time.
- `const` values have no memory or other storage associated with them, they are not places, they are copied into each place where they are referenced.
- The lifetime of a `const` value is tied to the scope in which it is defined.
- Example:

```rust
const MAX_POINTS: u32 = 100_000;
```

## `static` aka. global variable

- `static` defines a global variable that has a fixed memory location.
- It can be mutable if declared with the `mut` keyword, but mutable statics require unsafe code to access.
- `static` variables are initialized only once and exist for the entire duration of the program.
- The lifetime of a `static` variable is `'static`, meaning it lives for the entire duration of the program.
- Example:

```rust
static mut COUNTER: u32 = 0;
```

## Summary of Differences

| Feature         | `const`                       | `static`                            |
| --------------- | ----------------------------- | ----------------------------------- |
| Memory Location | Inlined at each use           | Fixed memory location               |
| Mutability      | Always immutable              | Can be mutable (with `mut`)         |
| Initialization  | Must be a constant expression | Can be initialized at runtime       |
| Lifetime        | Tied to scope                 | `'static` (entire program) duration |

In summary, use `const` for values that are constant and can be inlined, and use `static` for global variables that need a fixed memory location and may require mutability.
