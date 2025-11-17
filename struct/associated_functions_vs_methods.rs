struct Rectangle {
    width: u32,
    height: u32,
}

// Note: we can have multiple impl blocks for a struct, here we just use one for simplicity.
impl Rectangle {
    // Associated function
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Using the associated function to create a new Rectangle instance
    let rect = Rectangle::new(30, 50);

    // Using the method to calculate the area of the rectangle
    println!("The area of the rectangle is {} square pixels.", rect.area());
}

// Note: the comparison table between associated functions and methods
// | Aspect                 | Associated Functions                     | Methods                                  |
// |------------------------|------------------------------------------|------------------------------------------|
// | Definition             | Defined within an impl block using fn    | Defined within an impl block using fn with &self or &mut self |
// | Invocation             | Called using the struct name (e.g., StructName::function_name()) | Called on an instance of the struct (e.g., instance.method_name()) |
// | Purpose                | Typically used for constructors or utility functions that don't need access to instance data | Used to operate on instance data and can access the struct's fields via self |