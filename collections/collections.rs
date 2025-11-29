fn main() {
    vector_structure();
    vector_with_ownership();
    vector_use_enum_to_store_different_types();
    questions_about_vector();

    hash_map();
}

fn vector_structure() {
    // Note: Vec<T> is a smart pointer that owns a heap-allocated array of elements of type T.
    // (remember that a smart pointer is a pointer with additional metadata and capabilities)
    //
    // **capabilities**:
    // 1. Dynamic resizing: it can grow and shrink as elements are added or removed.
    // 2. Contiguous memory allocation: elements are stored in a contiguous block of memory on the heap.
    // 3. Random access: elements can be accessed in constant time using indexing.
    //
    // What is stored in stack and what is stored in heap?
    // **metadata**
    // The Vec<T> struct itself is stored on the stack, which contains three fields:
    // 1. A pointer to the heap-allocated array
    // 2. The length of the vector (number of elements currently stored)
    // 3. The capacity of the vector (total number of elements that can be stored without reallocating)
    // The actual elements of the vector are stored in a contiguous block of memory on the heap,
    // regardless of the type T. E.g., if T is i32, the heap will store a contiguous array of i32 values.

    // basic syntax
    // creation
    let mut v: Vec<i32> = Vec::new(); // standard way
    let mut v1 = vec![1, 2, 3]; // macro way
    // adding elements
    v.push(10);
    v.push(20);
    v.push(30);
    // accessing elements
    // v1[0] is a reference to the element at index 0, it's type is &i32
    let first = v1[0]; // panics if out of bounds
    let second = v1.get(1); // returns Option<&T>
    // iterating
    // immutable iteration
    for i in &v1 {
        println!("{}", i);
    }
    // mutable iteration
    for i in &mut v {
        *i += 1;
    }
    // iterator methods
    let mut v_ref = &mut v1;
    for num_ref in v_ref.iter() {
        // Note: v_ref.iter() immutably borrows v_ref for the duration of the loop.
        // v_ref.push(*num_ref); // this won't compile because we are trying to mutate v_ref while iterating over it
    }
}

fn vector_with_ownership() {
    // vector of Strings
    let mut v: Vec<String> = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ];
    // v[0] is a reference to the String at index 0, it's type is &String
    // since the vector is declared as mutable, we can mutate the String inside the vector
    // it's an in-place mutation, so the ownership of the String does not change.
    v[0].push('1');
    println!("{}", v[0]);

    let v1: Vec<String> = vec![
        String::from("x"),
        String::from("y"),
        String::from("z"),
    ];
    // let s: String = v1[0]; // this line causes a compile error because we are trying to move the ownership of the String out of the vector
    // Note: non-copyable types cannot be moved out of a vector by indexing.
}

fn vector_use_enum_to_store_different_types() {
    // Note: vectors only support storing elements of the same type.
    // To store different types in the same vector, we can use an enum to encapsulate the different types.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(20.5),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
}

fn questions_about_vector() {
    /*
    Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

    fn main() {
      let mut v: Vec<i32> = vec![1, 2, 3];
      let mut v2: Vec<&mut i32> = Vec::new();
      for i in &mut v {
        v2.push(i);
      }
      *v2[0] = 5;
      let a = *v2[0];
      let b = v[0];
      println!("{a} {b}");
    }
     */
    // Answer: the program will pass. The expected output is "5 5".
    // The key point here is that v2 is a vector of **mutable references** to the elements in v.
}

fn hash_map() {
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    // inserting key-value pairs
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    // accessing values - get returns an Option<&V>
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }
    // iterating over key-value pairs
    // Note: why do we use &scores here instead of scores?
    // because iter() borrows the HashMap immutably, so we need to pass a reference to it.
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Note: HashMap takes ownership of the keys and values inserted into it.
    let team_name = String::from("Charlie");
    let team_score = 30;
    scores.insert(team_name, team_score);
    // println!("{}", team_name); // this line would cause a compile error because team_name has been moved into the HashMap.

    // updating a value using the entry API
    scores.entry(String::from("Alice")).and_modify(|e| *e += 5).or_insert(0);
    println!("Alice's updated score: {}", scores.get("Alice").unwrap());
}