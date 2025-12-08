// RefCell is a smart pointer that provides interior mutability in Rust.
// It allows you to mutate data even when there are immutable references to that data.
// RefCell enforces Rust's borrowing rules at runtime rather than at compile time.
// At runtime, when the borrowing rules are violated, RefCell will **panic**.

// An example of using RefCell
use std::cell::RefCell;
fn main() {
    let data = RefCell::new(5);

    // Immutable borrow
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        println!("Immutable borrows: {}, {}", *borrow1, *borrow2);
        // borrow1 and borrow2 go out of scope here
    }

    // Mutable borrow
    {
        let mut borrow_mut = data.borrow_mut();
        *borrow_mut += 10;
        println!("Mutable borrow after modification: {}", *borrow_mut);
        // borrow_mut goes out of scope here
    }

    // Check the final value
    let final_borrow = data.borrow();
    println!("Final value: {}", *final_borrow);
}

// A use case of using RefCell - mocking in tests
// The limit tracker checks if a certain value exceeds predefined limits
// and sends messages via a Messenger trait object.
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    // To unit test the LimitTracker, we need a mock Messenger.
    use super::*;
    struct MockMessenger {
        // Note:
        // Why Vec<String> doesn't work here?
        // Notice that the send method takes &self, which is an immutable reference.
        // If we use Vec<String> directly, we won't be able to mutate it inside send.
        // RefCell provides interior mutability, allowing us to mutate the Vec even when we have an immutable reference to MockMessenger.
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // We can borrow a mutable reference to the Vec inside RefCell
            // even though self is an immutable reference.
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // Check that a warning message was sent
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(
            mock_messenger.sent_messages.borrow()[0],
            "Warning: You've used up over 75% of your quota!"
        );
    }
}

// Note: RefCell is not thread-safe.

// Note: With Rc supporting multiple ownership and RefCell enabling interior mutability,
// we can combine them to create data structures that allow multiple owners to mutate shared data.
// see an example in smart_pointers/ref_cycle.
