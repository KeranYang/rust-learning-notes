/*
Is rust an object-oriented programming language?

If we look at some of the core principles of OOP:
1. Objects contain both data and behavior. In Rust, structs can encapsulate data, and we can define methods on structs to provide behavior.
2. Encapsulation: Rust allows us to control the visibility of struct fields and methods using the pub keyword, enabling encapsulation.
3. Inheritance: Rust does not have traditional class-based inheritance. However, we can achieve code reuse through composition and traits.
4. Polymorphism: Rust supports polymorphism through traits, allowing different types to be treated uniformly based on shared behavior.
 */

// An example of using traits to achieve polymorphism

pub trait Draw {
    fn draw(&self);
}

// Implementing the Draw trait for different types
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {} ({}x{})", self.label, self.width, self.height);
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing a text field: {} ({}x{})", self.text, self.width, self.height);
    }
}

// Dynamic Dispatch:
// A Screen struct that holds a vector of components that implement the Draw trait.
pub struct Screen {
    // Note: the dyn keyword indicates that we are using a trait object,
    // which allows for dynamic dispatch at runtime.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Static Dispatch:
// An alternative way to define Screen using generics
// This approach uses static dispatch at compile time.
// Meaning that the specific type T must be known at compile time,
// and all components in the vector must be of the same type T.
// They can only be all Buttons, or all TextFields, etc.
pub struct AnotherScreenDefinition<T: Draw> {
    pub components: Vec<T>,
}

impl<T: Draw> AnotherScreenDefinition<T> {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(TextField {
                width: 100,
                height: 20,
                text: String::from("Enter your name"),
            }),
        ],
    };
    screen.run();
}