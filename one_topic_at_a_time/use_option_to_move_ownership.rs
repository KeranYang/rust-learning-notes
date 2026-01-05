// Note: one can use Option<T> to move ownership of T into a closure or another function.
//
// Why do we need to do this?
// Because Rust's closures capture variables by reference by default, which means
// we cannot move ownership of a variable into a closure if it is already borrowed.
// By wrapping the variable in an Option, we can use the take() method to move
// ownership of the variable out of the Option, leaving None in its place.
// This allows us to transfer ownership of the variable into the closure without
// violating Rust's borrowing rules.
//
// When should we use this pattern?
// We shouldn't overuse it. It's used only for cases when
// > “This value starts here, may be taken exactly once, and after that it is gone — and that is a meaningful state.”
//
// Here is an example:
fn main() {
    use_option_to_move_ownership();
}

fn use_option_to_move_ownership() {
    struct Resource {
        data: String,
    }
    impl Resource {
        fn new(data: &str) -> Self {
            Resource {
                data: data.to_string(),
            }
        }
        fn consume(self) {
            println!("Consuming resource with data: {}", self.data);
        }
    }
    let mut resource_option = Some(Resource::new("Important Data"));
    let mut closure = || {
        if let Some(resource) = resource_option.take() {
            resource.consume();
        } else {
            println!("Resource has already been consumed.");
        }
    };
    closure(); // Consuming resource with data: Important Data
    closure(); // Resource has already been consumed.
}
