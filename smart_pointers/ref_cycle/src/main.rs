/*
This example demonstrates a reference cycle using Rust's `Rc` and `RefCell`.
It creates two nodes that reference each other, leading to a memory leak
because the reference counts never reach zero.
*/
fn main() {
    simple_ref_cycle();
    ref_cycle_with_tree();
}

fn simple_ref_cycle() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: RefCell<Option<Rc<Node>>>,
    }

    let node1 = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let node2 = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&node1))),
    });

    // Creating a reference cycle
    *node1.next.borrow_mut() = Some(Rc::clone(&node2));

    // println!("Node 1: {:?}", node1); // This would cause infinite recursion.
    // println!("Node 2: {:?}", node2); // This would cause infinite recursion.

    // At this point, both node1 and node2 reference each other.
    // Their reference counts will never reach zero, leading to a memory leak.
    println!(
        "Reference count of node1: {}",
        Rc::strong_count(&node1) // 2
    );
    println!(
        "Reference count of node2: {}",
        Rc::strong_count(&node2) // 2
    );
}

fn ref_cycle_with_tree() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        // Note: Using Weak reference for parent to avoid reference cycle.
        // A Weak Reference does not contribute to the reference count.
        //
        // The key point is that a parent node should own its children, when a parent node is dropped,
        // its children should also be dropped.
        // However, children should not own their parent,
        // when a child node is dropped, its parent should not be dropped, this is a case for Weak Reference.
        parent: RefCell<Weak<TreeNode>>,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }

    let leaf = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(TreeNode {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Let's inspect the strong and weak counts
    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 2
        Rc::weak_count(&leaf)    // 0
    );
    println!(
        "Branch strong = {}, weak = {}",
        Rc::strong_count(&branch), // 1
        Rc::weak_count(&branch)    // 0
    );

    // Setting the parent of leaf to branch
    // the downgrade method creates a Weak reference from a Rc reference.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "After setting parent, Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 2
        Rc::weak_count(&leaf)    // 0
    );
    println!(
        "After setting parent, Branch strong = {}, weak = {}",
        Rc::strong_count(&branch), // 1
        Rc::weak_count(&branch)    // 1
    );

    // now, when would the memory be freed?
    // when branch goes out of scope, its strong count becomes 0, it is dropped
    // even if the leaf still has a strong count of 1, its parent is a Weak reference, so it does not prevent branch from being dropped.
}
