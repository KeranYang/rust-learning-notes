// 1. function pointers
// Note: Function pointers are used to point to functions defined in the code.
// Syntax: fn(parameter_types) -> return_type
// fn(i32) -> i32
fn add_one(x: i32) -> i32 {
    x + 1
}

// f is a function pointer.
fn apply_function(f: fn(i32) -> i32, value: i32) -> i32 {
    f(value)
}

fn main() {
    let result = apply_function(add_one, 5);
    println!("Result: {}", result); // Output: Result: 6
}

// 2. returning closures
// Note: since the closure implements trait, it is a trait object.
// We cannot return a closure directly because its size is not known at compile time.
// The solution is to put it behind a pointer, such as Box.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}