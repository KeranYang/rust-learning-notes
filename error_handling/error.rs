// Note: There are two types of errors in Rust: recoverable and unrecoverable errors.
// Recoverable errors are represented using the Result<T, E> enum, while unrecoverable
// errors typically cause the program to panic and terminate.

// Note: main function cannot return Result<T, E>.
use std::error::Error;
// Box<dyn Error> is a trait object that can hold any error type that implements the Error trait.
fn main() -> Result<(), Box<dyn Error>> {
    recoverable_error_example();
    unrecoverable_error_example();
    Ok(())
}

fn recoverable_error_example() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        // Note: the ? operator is a shorthand for **propagating errors**.
        // If the Result is Ok, it unwraps the value inside and continues.
        // If the Result is Err, it returns the error from the current function.
        // To use it, the function must
        // a. return a Result type and
        // b. the error type must match the error type of the Result being propagated.
        let mut f = File::open("username.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // This function demonstrates the more verbose way of handling errors without using the ? operator.
    fn open_a_file() -> Result<String, io::Error> {
        let f = File::open("username.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => match File::create("username.txt") {
                    Ok(fc) => fc,
                    Err(e) => return Err(e),
                },
                _ => return Err(io::Error::new(e.kind(), "Problem opening the file")),
            },
        };
        // Continue reading the file...
        Ok(String::new())
    }

    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
}

fn unrecoverable_error_example() {
    let v = vec![1, 2, 3];
    // This will cause a panic because we are trying to access an out-of-bounds index.
    println!("Accessing out-of-bounds element: {}", v[99]);

    // Note: panic! macro is used to cause a program to terminate when it encounters an unrecoverable error.
    // It prints the provided message and unwinds the stack.
    // explicitly causing a panic
    panic!("This is a panic message");

    // **unwrap** to cause a panic if the Result is an Err variant.
    let file = std::fs::File::open("non_existent_file.txt").unwrap();

    // **expect** to cause a panic with a custom message if the Result is an Err variant.
    // Note: in production code, prefer using expect over unwrap for better error messages with context.
    let file2 =
        std::fs::File::open("another_non_existent_file.txt").expect("Failed to open the file");
}
