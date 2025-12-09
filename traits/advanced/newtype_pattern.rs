// Note: Orphan rule - A trait can be implemented for a type only if either the trait or the type is local to the current crate.
// We cannot implement external traits for external types directly.
// This is because it could lead to conflicts if multiple crates try to implement the same trait for the same type.
// Note: we can work around this limitation by using the newtype pattern,
// which involves creating a new type that wraps the external type.

// Example: Implementing the Display trait for Vec<T> using the newtype pattern.
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
        String::from("from"),
        String::from("Rust"),
    ]);
    println!("{}", w);
}

// Note: the downside of the newtype pattern is that we can only use the methods defined on the new type.
// If we were to use all the methods of the inner type, we would need to implement Deref and DerefMut traits for the new type to delegate method calls to the inner type.