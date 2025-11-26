// What is a closure in Rust?
// A closure in Rust is an anonymous function that can capture variables from its surrounding scope.
fn main() {
    simple_closure_example();
    closures_can_infer_types();
    closures_with_borrowing();
    closures_taking_ownership();
    fn_once_fn_mut_fn_example();
    iterators_example();
}

// simple closures that capture variables from their environment
fn simple_closure_example() {
    // case 1: a closure that takes no parameters
    let greet = || println!("Hello, world!");
    greet();

    // case 2: a closure that takes parameters
    let add = |a: i32, b: i32| a + b;
    let result = add(5, 10);
    println!("Sum: {}", result);

    // case 3: a closure with multiple statements
    let multiply = |a: i32, b: i32| {
        let product = a * b;
        product
    };
    println!("Multiply: {}", multiply(5, 10));
}

fn closures_can_infer_types() {
    // Demonstrate diffs between fn and closure
    // Note: closures can infer types.
    fn add_one_v1 (x: i32) -> i32 {
        x + 1
    }
    let add_one_v2 = |x| x + 1;

    // Note: closures can infer types, but only once.
    let result_1 = add_one_v2(5); // x is inferred as i32
    // let result_2 = add_one_v2(5.5); // -> compile error: expected i32, found floating-point number
}

// closures that borrow variables from their environment
fn closures_with_borrowing() {
    // case 1: borrowing an immutable variable
    let list = vec![1, 2, 3, 4, 5];
    let sum = || -> i32 {
        list.iter().sum::<i32>() // list is borrowed immutably
    };
    let result = sum();
    println!("Result: {}", result);
    println!("list: {:?}", list); // list is still accessible here because it was borrowed immutably

    // case 2: borrowing a mutable variable
    let mut list2 = vec![10, 20, 30];
    let mut add_to_list = |value: i32| {
        list2.push(value); // list2 is borrowed mutably
    };
    // println!("list2 before: {:?}", list2); // compile error: cannot borrow `list2` as immutable because it is also borrowed as mutable
    add_to_list(40);
    println!("list2: {:?}", list2); // list2 is still accessible here because the closure has finished executing.
}

// closures that take ownership of variables from their environment
fn closures_taking_ownership() {
    // case 1: a general example of a closure taking ownership
    let s = String::from("hello");
    let consume_string = || {
        println!("Consumed string: {}", s); // s is moved into the closure
    };
    consume_string();
    // when we reach here, s is no longer valid because its ownership has been moved to the closure
    // println!("s: {}", s); // -> compile error: borrow of moved value: `s`

    // case 2: using the `move` keyword to force a closure to take ownership, used in multi-threading
    use std::thread;
    let s2 = String::from("hello from thread");
    // the thread needs to own s2, so we use `move` to transfer ownership into the closure.
    // what does `move` do here?
    // it forces the closure to take ownership of ANY variables it uses from its environment.
    // in this case, since the thread only uses s2, only s2 is moved into the closure.
    let handle = thread::spawn(move || {
        println!("{}", s2); // s2 is moved into the closure
    });
    handle.join().unwrap();
    // println!("{s2}"); // -> compile error: borrow of moved value: `s2`
}

fn fn_once_fn_mut_fn_example() {
    // Note: closures implement one of the three traits: Fn, FnMut, or FnOnce,
    // depending on how they capture variables from their environment.

    // Fn closure - can be called multiple times without mutating or taking ownership.
    let x = 5;
    let fn_closure = || println!("Fn closure: x = {}", x);
    fn_closure();
    fn_closure();

    // FnMut closure - can mutate captured variables, requires mutable access.
    let mut y = 10;
    let mut fn_mut_closure = || {
        y += 5;
        println!("FnMut closure: y = {}", y);
    };
    fn_mut_closure();
    fn_mut_closure();

    // FnOnce closure - can only be called once.
    let z = String::from("Hello");
    let fn_once_closure = || {
        println!("FnOnce closure: z = {}", z);
        // z is moved into the closure, hence no longer available in its original scope.
    };
    fn_once_closure(); // after this call, z is out of scope.
    // fn_once_closure(); // -> compile error: closure cannot be invoked more than once because it moves the variable `z`

    // Note: all closures implement FnOnce because they can all be called at least once.

    // Real World example - unwrap_or_else is designed to take a closure F that implements FnOnce,
    // For two reasons:
    // 1. unwrap_or_else only needs to call the closure once to get the default value.
    // 2. every closure implements FnOnce, so accepting FnOnce makes the function more flexible.
    // impl <T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T,
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }
    let opt: Option<String> = None;
    let value = opt.unwrap_or_else(|| String::from("Default Value"));
    println!("Value: {}", value);
}

fn iterators_example() {
    let vec = vec![1, 2, 3, 4, 5];

    // consuming adaptor - method that consumes the iterator
    let vec_iter = vec.iter();
    let sum: i32 = vec_iter.sum();
    println!("Sum: {}", sum);
    // we are not allowed to use vec_iter here because it has been **consumed by sum()**
    // println!("{:?}", vec_iter); // -> compile error: use of moved value: `vec_iter`

    // producing adaptor - method that produces a new iterator
    // **map() produces** a new iterator by applying a closure to each element
    let squared: Vec<i32> = vec.iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);

    // closure that captures its environment
    let multiplier = 3;
    // multiplier is captured by the closure
    let multiplied: Vec<i32> = vec.iter().map(|x| x * multiplier).collect();
    println!("Multiplied by {}: {:?}", multiplier, multiplied);

    // Using a closure with the `for_each` iterator adaptor to print each element
    vec.iter().for_each(|x| println!("Element: {}", x));

    // Note:
    // Iterators are lazy - they do not perform any computation until they are consumed by a consuming adaptor.
    // Iterators can make code cleaner.
    // Performance - Rust's iterators are zero-cost abstractions, meaning they have no runtime overhead compared to manual loops.
}