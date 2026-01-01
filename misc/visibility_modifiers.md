# Access Visibility Modifiers in Rust

There are four main visibility modifiers in Rust:
1. `pub`: The item is public and can be accessed from any module, both within the same crate and from external crates.
2. `pub(crate)`: The item is public within the current crate, but not outside of it.
3. `pub(super)`: The item is public to the parent module (the module that contains the current module).
4. Private (no modifier): The item is private and can only be accessed within the same module.

By default, items in Rust are private. You need to explicitly use the `pub` modifier to make them accessible from other modules or crates.