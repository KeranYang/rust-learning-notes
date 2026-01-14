# Customize builds with Release Profiles

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Note: opt-level controls the number of optimizations. For dev, we prefer fast compilation over runtime performance, hence opt-level = 0. For release builds, we want maximum performance, so we set opt-level = 3.

# What is crate?

In Rust, a crate is the smallest unit of code distribution and compilation.
It can be thought of as a package or library that contains Rust source code.
A crate can be either a binary crate, which produces an executable program,
or a library crate, which provides reusable functionality that can be included in other crates.

# Publishing a crate to crates.io

## Making useful documentation comments

- Use `///` and it supports Markdown formatting.
- Run `cargo doc --open` to generate and view the documentation locally.
- Document Panics, Errors, and Safety considerations. If unsafe, explain why and how to use it safely.
- Documentation comments as tests - running `cargo test` will also test the code examples in your documentation comments.

````
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let sum = my_crate::add(2, 3);
/// assert_eq!(sum, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

- Use `//!` for module-level documentation.

## Publishing

- Ensure your `Cargo.toml` has the necessary metadata: name, version, authors, description, license, repository, etc.
- `cargo publish --dry-run` to check for issues before publishing.
- Run `cargo publish` to publish your crate to crates.io.

# Cargo Workspaces

A cargo workspace is a set of packages that share the same Cargo.lock and output directory.

Workspaces are useful for managing multiple related packages in a single repository.

Take a look at the add folder for an example of a cargo workspace with two packages:
a binary crate `adder` and a library crate `add_one`, which is used by `adder`.

# Installing Binaries with `cargo install`

You can use `cargo install` to install binary crates from crates.io or from a local path. E.g., to install the `ripgrep` tool:

```
cargo install ripgrep
```

# cargo.toml

## features

```toml
tokio = { version = "1.0", features = ["full"] }
```

`features` allow you to enable optional functionality in a crate. In this case, the `full` feature enables all optional components of the `tokio` crate.

When attempting to optimize compile time or the end application footprint,
the application can decide to opt into only the features it uses.
An example would be:

```toml
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

## bins

We can use the `[[bin]]` section to define binary crates within a workspace. See examples in the `adder` folder.

When to use and when not to use `[[bin]]`?

| Situation                       | Use `[[bin]]`? |
| ------------------------------- | -------------- |
| Single binary in `src/main.rs`  | ❌ No          |
| Multiple binaries in `src/bin/` | ❌ No          |
| Custom binary path              | ✅ Yes         |
| Custom binary name              | ✅ Yes         |
| Disable tests/bench/docs        | ✅ Yes         |
| CLI + daemon in same crate      | ✅ Yes         |
