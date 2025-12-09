// 1. Note: newtype pattern.
// Use newtype pattern to
// a) implement external traits for external types
// b) create type-safe wrappers to avoid confusion between similar types
// c) exercise abstraction by hiding implementation details
// d) indicate unit of measurement
// etc.

// 2. Note: type aliasing.
// Use type aliasing to
// a) simplify complex type signatures
// b) improve code readability
// c) enforce consistency in type usage
type Thunk = Box<dyn Fn() + Send + 'static>;

// 3. Note: !: the never type that never returns.
fn bar() -> ! {
    panic!("This function never returns!");
}

fn main() {
    loop {
        let guess: u32 = match "42".parse() {
            Ok(num) => num,
            Err(_) => {
                // Note: continue is one of the expressions that has the never type.
                // Since it never returns a value, the compiler won't complain about type mismatches
                // across the match arms.
                continue;
            },
        };
    }
    bar();
}

// 4. Note: DST - Dynamically Sized Types
// Note: most types in Rust have a known size at compile time.
// However, some types, known as Dynamically Sized Types (DSTs),
// do not have a fixed size at compile time.
// Example of DSTs:
// 1. str (string slice)
// 2. [T] (slice of T)
// 3. Trait objects (e.g., dyn Trait)
// The golden rule for DSTs is that they must be used behind some kind of pointer.
//
// Sized trait is automatically implemented for types with a known size at compile time.
// By default, generic type parameters are assumed to be Sized.
fn generic_function<T>(t: T) {}
// This function is equivalent to:
fn generic_function_sized<T: Sized>(t: T) {}
//
// If we want to allow DSTs as generic parameters,
// we can use the special syntax T: ?Sized to indicate that T may or may not be Sized.
fn generic_function_dsts<T: ?Sized>(t: &T) {}
// Note: the ?Trait syntax with the meaning of "may or may not implement Trait" is only applicable to Sized trait.
// Note: see that we also change the parameter type to &T,
// because DSTs must be used behind some kind of pointer.