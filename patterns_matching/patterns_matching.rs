fn main() {
    match_arms_example();
    if_let_example();
    while_if_let_example();
    let_is_pattern_matching();
    fn_parameter_pattern_matching();
    refutable_irrefutable_patterns();
    extra_conditional_checks_with_match_guards();
    the_at_sign_pattern();
    other_pattern_matching_examples();
}

fn match_arms_example() {
    // Note: match arms need to be exhaustive.
    // Note: match expression evaluates arms from top to bottom and stops at the first matching arm.
    let number = Some(7);
    match number {
        Some(1) => println!("One"),
        Some(2) => println!("Two"),
        Some(3) => println!("Three"),
        Some(n) if n > 3 && n < 10 => println!("A number between four and nine: {}", n),
        Some(n) => println!("A number greater than or equal to ten: {}", n),
        None => println!("No number"),
    }
}

fn if_let_example() {
    // Note: `if let` is useful when we only care about one pattern.
    // The downside is the compiler does not check for exhaustiveness.
    let some_value: Option<i32> = Some(10);
    if let Some(10) = some_value {
        println!("The value is ten!");
    }
}

fn while_if_let_example() {
    // Note: `while let` can be used for looping with pattern matching.
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 5 {
            println!("Greater than 5, stopping.");
            optional = None;
        } else {
            println!("i is: {}", i);
            optional = Some(i + 1);
        }
    }
}

fn let_is_pattern_matching() {
    // Note: `let` can also be used for pattern matching.
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn fn_parameter_pattern_matching() {
    // Note: function parameters can also use pattern matching.
    fn print_point((x, y): (i32, i32)) {
        println!("Point coordinates: x = {}, y = {}", x, y);
    }
    let point = (10, 20);
    print_point(point);
}

fn refutable_irrefutable_patterns() {
    // Note: irrefutable patterns always match.
    let x = 5;
    let y = x; // irrefutable pattern

    // Note: refutable patterns may not match.
    let some_option: Option<i32> = Some(10);
    if let Some(value) = some_option { // refutable pattern
        println!("The value is: {}", value);
    }

    let x: &[(i32, i32)] = &[(2, 1)];
    // Which of the following are refutable patterns?
    // 1. let [(a, b)] = x; // refutable
    // 2. let [(a, b), ..] = x; // refutable
    // 3. let _ = x; // irrefutable
    // 4. let &[..] = x; // irrefutable
    // This is because 1 and 2 assumes that x has at least one element,
    // while 3 and 4 will always match regardless of the contents of x.
}

fn extra_conditional_checks_with_match_guards() {
    // Note: match guards add extra conditional checks to patterns.
    let number = Some(7);
    match number {
        Some(n) if n < 5 => println!("Less than five: {}", n),
        Some(n) if n >= 5 && n < 10 => println!("Between five and nine: {}", n),
        Some(n) => println!("Ten or greater: {}", n),
        None => println!("No number"),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("x is 4, 5, or 6 and y is true"),
        _ => println!("Other case"),
    }
    // Note: when match guards are used with multiple patterns (like 4 | 5 | 6),
    // the guard applies to the entire pattern, not just the last one.
    // 4 | 5 | 6 if y means (4 | 5 | 6) and y.
}

fn the_at_sign_pattern() {
    // Note: the @ sign allows us to bind a value to a variable while also matching it against a pattern.
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        // Note:
        // the @ sign binds the matched value to id_variable if it falls within the range 3 to 7,
        // INCLUSIVELY.
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range 3 to 7: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in range 10 to 12")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

fn other_pattern_matching_examples() {
    // Other pattern matching examples

    // 1. Multiple patterns with |
    let value = 2;
    match value {
        1 | 2 | 3 => println!("Value is one, two, or three"),
        _ => println!("Other value"),
    }

    // 2. Matching ranges
    let age = 25;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        _ => println!("Senior"),
    }

    // 3. Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };
    match p {
        Point { x, y } => println!("Point coordinates: x = {}, y = {}", x, y),
        _ => (),
    }

    // 4. Destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    // 5. the _ wildcard pattern
    // 5.1 ignoring values
    let numbers = (1, 2, 3);
    match numbers {
        (x, _, z) => println!("x: {}, z: {}", x, z),
    }
    fn foo(_: i32, y: i32) {
        println!("y: {}", y);
    }
    foo(10, 20); // only y is used
    // 5.2 unused variables starting with _
    let _unused_variable = 42; // no compiler warning

    // 6. the .. syntax for ignoring remaining values
    struct Rectangle {
        width: u32,
        height: u32,
        color: String,
    }
    let rect = Rectangle {
        width: 30,
        height: 50,
        color: String::from("blue"),
    };
    match rect {
        Rectangle { width, .. } => println!("Width: {}", width),
    }
    let numbers = [1, 2, 3, 4, 5];
    match numbers {
        [first, .., last] => println!("First: {}, Last: {}", first, last),
    }
}