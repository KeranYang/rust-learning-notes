#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        if width == 0 || height == 0 {
            panic!("Width and height must be greater than zero");
        }
        Rectangle { width, height }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
// Note: this annotation tells Rust to compile and run the test code only when we run cargo test.
// It also tells that the code inside this module shouldn't be included in the final build of the library or binary.
// More on cfg attribute:
// the attribute cfg stands for "configuration" and is used for conditional compilation.
// only compile the code inside this module when the "test" configuration is active, aka when we run cargo test.
mod tests {
    use super::*;

    #[test]
    fn large_can_hold_small() {
        let large = Rectangle::new(8, 7);
        let small = Rectangle::new(5, 1);
        // we can add an optional custom failure message to assert macros
        assert!(large.can_hold(&small), "Large rectangle should be able to hold small rectangle");
    }

    #[test]
    fn small_cannot_hold_large() {
        let large = Rectangle::new(8, 7);
        let small = Rectangle::new(5, 1);
        assert!(!small.can_hold(&large));
    }

    #[test]
    #[should_panic(expected = "Width and height must be greater than zero")]
    // Note: test using should_panic should be precise about the expected panic message
    // otherwise, if the code panics for a different reason, the test will still pass.
    fn create_rectangle_with_zero_dimension() {
        Rectangle::new(0, 5);
    }

    #[test]
    // Note: we can also use Result<T, E> in tests.
    // This test is just to demonstrate that feature.
    fn test_using_result() -> Result<(), String> {
        let rect = Rectangle::new(3, 4);
        if rect.width != 3 {
            return Err(String::from("Width is not correct"));
        }
        Ok(())
    }

    #[test]
    #[ignore]
    // Note: ignored tests will not be run by default.
    // To run ignored tests, use the command: cargo test -- --ignored
    fn ignored_test() {
        let rect = Rectangle::new(1, 1);
    }
}
