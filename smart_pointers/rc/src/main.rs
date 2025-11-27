// Note: what is Rc<T> in Rust?
// The Rc<T> type is a reference-counted smart pointer that enables multiple ownership of data in Rust.
// It keeps track of the number of references to the data it points to, allowing multiple parts of a program to share ownership of the same data.
// When the last reference to the data goes out of scope, the data is automatically deallocated.

// Note: when to use Rc<T>?
// We want to use multiple ownership of heap data, but we don't know at compile time which owner will finish using the data last.
// (If we know, we could just assign single ownership to that last owner.)

// Note: Rc<T> is used in single-threaded scenarios only.
use std::rc::Rc;
fn main() {
    problem_statement();
    solution_with_rc();
}

fn problem_statement() {
    // Let's say we have a list structure, where multiple nodes can point next to the same node.
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // -> compile error because the ownership of a is moved here
    // let c = Cons(4, Box::new(a)); // -> compile error because the ownership of a is moved here
    // To solve this problem, we can use Rc<T> to enable multiple ownership.
}

fn solution_with_rc() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Note: Rc::clone(&a) creates a new Rc pointer to the same data, increasing the reference count.
    // Rc:clone() is cheap, it only increases the reference count, hence doesn't introduce performance overhead.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // Rc::strong_count(&a) returns the number of strong references to the data.
    // a strong reference is a Rc<T> pointer that owns the data.
    println!("a count after creating b and c = {}", Rc::strong_count(&a)); // should be 3
    {
        let d = Cons(6, Rc::clone(&a));
        println!("a count after creating d = {}", Rc::strong_count(&a)); // should be 4
    } // d goes out of scope here
    println!("a count after d goes out of scope = {}", Rc::strong_count(&a)); // should be 3
    // Note: when all Rc pointers to the data go out of scope, the data is automatically deallocated.
}