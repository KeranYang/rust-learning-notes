struct Rectangle {
    width: u32,
    height: u32,
}

// Note: we can have multiple impl blocks for a struct, here we just use one for simplicity.
impl Rectangle {
    // Associated function
    // Note: rust does not have a keyword for constructors for struct, new is just a convention.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Note: use reference &self as opposed to self to avoid taking ownership of the instance.
    // If we use self, the original instance will become invalid after calling this method.
    fn increase_width(&mut self, increment: u32) {
        self.width += increment;
    }
}

fn main() {
    // Using the associated function to create a new Rectangle instance
    let mut rect = Rectangle::new(30, 50);

    // Using the method to calculate the area of the rectangle
    // Note: the area method takes &self instead of self even though the caller is not a reference.
    // This is a syntactic sugar that when rect.area() is called, rust translates it to Rectangle::area(&rect)
    // rust will do the same if the caller is a mutable reference, e.g., (&mut rect).area() -> Rectangle::area(&rect)
    println!("The area of the rectangle is {} square pixels.", rect.area());

    rect.increase_width(30);
    println!("The new width of the rectangle is {} pixels.", rect.width);
}

// Note: the comparison table between associated functions and methods
// | Aspect                 | Associated Functions                     | Methods                                  |
// |------------------------|------------------------------------------|------------------------------------------|
// | Definition             | Defined within an impl block using fn    | Defined within an impl block using fn with &self or &mut self |
// | Invocation             | Called using the struct name (e.g., StructName::function_name()) | Called on an instance of the struct (e.g., instance.method_name()) |
// | Purpose                | Typically used for constructors or utility functions that don't need access to instance data | Used to operate on instance data and can access the struct's fields via self |