// Depend on the add_one crate in the same workspace.
use add_one;

fn main() {
    let result = add_one::add_one(5);
    println!("5 + 1 = {}", result);
}