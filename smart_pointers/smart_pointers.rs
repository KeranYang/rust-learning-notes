fn main() {
    box_example();
    deref_coercion_example();
}

/*
What is a smart pointer in Rust?
Data structures acting like a **pointer** but also have **additional metadata** and **capabilities**.
E.g.,
String stores its capacity metadata and has the capability to ensure its data is always valid UTF-8.

Regular references v.s. Smart pointers
- Regular references (&T and &mut T) are simple pointers that borrow data without taking ownership.
- Smart pointers, in many cases, own the data they point to.

Examples of Smart Pointers in Rust - Box, String, Vec, etc.

Smart Pointers implement the Deref and Drop traits
- Deref trait allows smart pointers to behave like regular references.
- Drop trait allows smart pointers to have custom cleanup logic when they go out of scope.

When to use Smart Pointers
- When you have a type whose size is not known at compile time or is too large to store on the stack,
and you want to use a value of that type in a context that requires a fixed size.
- When you have a large amount of data, and you want to transfer ownership without copying the data.
- When you want to own a value,
and you care only that it's a type that implements a particular trait rather than a specific type.

Now, let's use examples to illustrate the above statements, one by one.
 */

// Example 1: Using Box<T> to store recursive data types, which size is not known at compile time.
// The following definition of List is recursive and we cannot determine its size at compile time.
// enum List {
//     Cons(i32, List), // compile error: recursive type has infinite size
//     Nil,
// }
// To fix this, we can use Box<T> to store the recursive part on the heap.
enum List {
    // Box has a fixed size.
    Cons(i32, Box<List>),
    Nil,
}

// Box provide the capability of **indirection** and **heap allocation**.
fn box_example() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    let x = Box::new(5);
    assert_eq!(*x + 5, 10); // dereference the Box to get the value inside
    // why we can use *x to dereference the Box?
    // because Box<T> implements the Deref trait.
}

// The following example show how Deref trait is implemented and works with smart pointers.
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    // the deref method returns a reference because we don't want to mess with the ownership.
    // if we return T directly, it would move the value out of the MyBox, which is not what we want.
    fn deref(&self) -> &T {
        &self.0
    }
}
fn deref_example() {
    let y = MyBox::new(5);
    assert_eq!(5, *y);
    // Note: when we use *y, Rust actually calls *(y.deref()).
}

// Note: Deref coercion is a convenience feature that Rust provides to automatically convert
// references to types that implement the Deref trait into references to the type they point to.

// Implicit deref coercion with functions
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercion_example() {
    let m = MyBox::new(String::from("Rust"));
    // a sequence of deref coercions occurs here to make the types match.
    // &MyBox<String> is coerced to &String, then &String is coerced to &str.
    // because the deref method of MyBox<String> returns &String,
    // and String also implements Deref to &str.
    hello(&m);
}

// Note: last but not least about Deref coercion,
// Rust can deref from an immutable reference to another immutable reference,
// or from a mutable reference to another mutable reference.
// It can also deref from a mutable reference to an immutable reference,
// but NOT from an immutable reference to a mutable reference.

// Drop trait, which allows us to customize the behavior when a value goes out of scope.
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // custom cleanup logic here
        println!("I am automatically called when the MyBox is out of scope: Dropping MyBox containing!");
    }
}
// Note: Rust doesn't allow us to call drop explicitly.
// Once the Drop trait is implemented for a type, the ownership system will automatically call the drop method
// when the value goes out of scope.

