# Use `r#` for Raw Identifiers and Raw String Literals

1. Use `r#` before a raw identifier to use keywords as identifiers. (Not Really Recommended)

```rust
let r#fn = "This is a raw identifier";
println!("{}", r#fn);
```

2. Use `r#` before a raw string literal to include special characters without escaping.

```rust
let raw_str = r#"This is a "raw" string with no need to escape quotes!"#;
println!("{}", raw_str);
```