fn main() {
    basic_ownership_example();
    clone_avoids_move();
    owner_can_modify();
    rust_evaluate_every_branch();
}

fn basic_ownership_example() {
    println!("--- when a primitive type is assigned to another variable, ownership is copied instead of moved ---");
    let x = 5; // x owns the value 5
    let y = x; // y gets a copy of the value, x is still valid
    println!("x = {}, y = {}", x, y); // both x and y are valid

    println!("--- when a heap-allocated type is assigned to another variable, ownership is moved ---");
    let s1 = String::from("hello"); // s1 owns the String data
    let s2 = s1; // ownership of the String data is moved to s2
    // println!("s1 = {}", s1); // -> compile error because s1 is no longer valid
    println!("s2 = {}", s2); // s2 is valid

    println!("--- when a type is passed to a function, ownership is moved ---");
    let s3 = String::from("world");
    take_ownership(s3); // ownership of s3 is moved to the function
    // println!("s3 = {}", s3); // -> compile error because s3 is no longer valid

    // Note: Moved heap data principle - when ownership of heap data is moved,
    // the original variable can no longer be used to access that data.
}

fn clone_avoids_move() {
    // Note: clone creates a deep copy of heap data to avoid ownership move.
    println!("--- using clone() to create a deep copy and avoid ownership move ---");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn owner_can_modify() {
    // Note: when ownership is moved, the new owner can modify the data.
    println!("--- the owner of heap data can modify it ---");
    let s = String::from("hello");
    let mut s1 = s; // ownership of s is moved to s1
    s1.push_str(", world!");
    println!("s1 = {}", s1);
    // s is no longer valid here because its ownership is moved to s1.
    // println!("s = {}", s); // -> compile error

    // Note: a function that takes ownership can also modify the data.
    let s2 = String::from("goodbye");
    let s3 = add_suffix(s2);
    println!("s3 = {}", s3);
    // s2 is no longer valid here because its ownership is moved to add_suffix.
    // println!("s2 = {}", s2); // -> compile error
}

fn rust_evaluate_every_branch() {
    // Note: rust checks ownership in every branch of conditional statements, even if some branches are never taken during runtime.
    let s1 = String::from("hello");
    let s2;
    if false {
        // although during runtime this branch is never taken,
        // rust still considers the ownership could be moved here,
        // hence, fail to compile.
        s2 = s1;
    } else {
        println!("In else branch, s1 is still valid: s1 = {}", s1);
    }
    // println!("s1 = {s1}"); // -> compile error because s1's ownership could be moved in the if branch.
}

fn take_ownership(s: String) {
    println!("Inside take_ownership: s = {}", s);
}

fn add_suffix(mut s: String) -> String {
    // here add_suffix takes ownership of s and modifies it
    s.push_str(" suffix");
    // then returns ownership back to the caller
    s
}