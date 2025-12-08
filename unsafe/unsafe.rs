// Note: why unsafe rust?
// 1. the static compile analysis is conservative.
// 2. rust, as a system programming language, needs to allow us to do low-level programming.

// Note: the **unsafe** keyword
// By using the unsafe keyword, we explicitly tell the compiler that we are aware of the potential risks.
// Keep the unsafe block small.

fn main() {
    // for better performance and for interfacing with other programming languages like C.
    deref_raw_pointers();
    // for calling an unsafe function or method.
    call_unsafe_functions();
    // for accessing or modifying a mutable static variable.
    // for implementing an unsafe trait.
    // for accessing fields of a union.
}

fn deref_raw_pointers() {
    let mut num = 5;
    let r1 = &num as *const i32; // immutable raw pointer. Note: **as** for casting
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn call_unsafe_functions() {
    // 1. a simple example
    unsafe fn dangerous() {
        println!("This is an unsafe function!");
    }

    // Calling an unsafe function requires an unsafe block.
    unsafe {
        dangerous();
    }
    // 2. we can create a safe abstraction using unsafe code.
    let mut arr = [1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut arr, 3);
    println!("left: {:?}, right: {:?}", left, right);

    // 3. using extern functions to call external code that could be unsafe.
    unsafe {
        let result = abs(-10);
        println!("The absolute value of -10 is: {}", result);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= slice.len());
    let ptr = slice.as_mut_ptr();
    // without unsafe, we would implement it this way
    // (&mut slice[..mid], &mut slice[mid..])
    // but this would cause a compile error because we cannot have two mutable references to the same data at the same time.
    // the compiler is not smart enough to understand that the two slices do not overlap.
    // unsafe to save us!
    // Note: this function is safe because we know 100% that the two slices do not overlap.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), slice.len() - mid),
        )
    }
}
