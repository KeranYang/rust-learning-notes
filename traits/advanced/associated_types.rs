// Note: use associated types in traits to define related types for implementers.
// The main benefit is that we can define a trait that uses some types without
// needing to know exactly what those types are until the trait is implemented.
pub trait Iterator {
    type Item; // associated type
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // specify the associated type
    fn next(&mut self) -> Option<Self::Item> {
        // implementation details...
        None
    }
}

// Why not use generics here?
//
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
//
// Using generics means that Counter can implement Iterator for multiple types T,
// leading to potential ambiguity.
//
// impl Iterator<u32> for Counter { ... }
// impl Iterator<String> for Counter { ... }
//
// Associated types tie the type directly to the implementer, making it clearer and more straightforward.
// E.g., for Counter, it will be one and only one type of Item, which is u32 in this case.

// A dummy main to make rustc happy.
fn main() {
    let mut counter = Counter { count: 0 };
    while let Some(value) = counter.next() {
        println!("Next value: {}", value);
    }
}