//! Cargo Demo
//! ``cargo_demo` is a simple Rust library that provides basic arithmetic functions.

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let sum = cargo_demo::add(2, 3);
/// assert_eq!(sum, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}