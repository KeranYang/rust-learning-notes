// Note: supertrait - if we have a use case where to implement a trait A,
// the type must also implement another trait B,
// then we call trait B a supertrait of trait A.

// An example, an OutlinePrint trait that requires the type to also implement the Display trait.
use std::fmt::Display;

pub trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", " ".repeat(len));
        println!("* {} *", output);
        println!("* {} *", " ".repeat(len));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {
    // No need to implement outline_print since it has a default implementation
}

// Implement Display for Point to satisfy the supertrait requirement
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    p.outline_print();
}