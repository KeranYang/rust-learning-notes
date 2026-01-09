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

# Use [] in the comment

Square brackets, when used in rust documentation comments, create intradoc-links to other items in the code base.

E.g., Using a proper code editor, one can apply `go to definition` to jump to the definition of [MapMode] from the comment below.

```rust
/// get_map_mode returns the [MapMode] if the server is a mapper.
/// None if the server is not a mapper.
pub fn get_map_mode(&self) -> Option<MapMode>
...
```
