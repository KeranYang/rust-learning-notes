// where does enum live in memory? stack or heap?

fn main() {
    simple_enum_example();
    option_enum_example();
    pattern_matching_example();
    advanced_pattern_matching_example();
    ownership_with_pattern_matching_example();
    unwrap_takes_ownership();
}

fn simple_enum_example() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    let m2 = Message::Move { x: 20, y: 80 };
}

fn option_enum_example() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // Note: we cannot add a number and an Option<i32> directly
    let x: i8 = 5;
    let y: Option<i8> = Some(10);
    // let sum = x + y; // -> compile error - you cannot add i8 and Option<i8>
    // such mechanism helps prevent null pointer exceptions at compile time.
    // you have to explicitly handle the case when the value is absent (None).

    // To use the value inside the Option, we need to match on it or use methods like unwrap, expect, etc.
    let sum = x + y.unwrap_or(0);
}

fn pattern_matching_example() {
    // Note: the match expression is exhaustive.
    let ip = IpAddr::V4(192, 168, 1, 1);

    // normal pattern matching with match
    match ip {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 Address: {}", addr);
        }
    }
    let number = Some(5);
    // pattern match with Option enum
    let incremented = plus_one(number);
    println!("Incremented value: {:?}", incremented);

    let some_value: Option<i32> = Some(10);
    match some_value {
        // pattern match with an exact value
        Some(10) => println!("The value is ten!"),
        Some(v) => println!("The value is: {}", v),
        // pattern match with wildcard _
        _ => println!("No value found"),
    }
}

fn advanced_pattern_matching_example() {
    // Note: the pattern matching evaluates the arms from top to bottom.
    // and stops at the first matching arm.
    // the wildcard pattern _ matches any value.
    let msg = Message::ChangeColor(10, 20, 30);
    match msg {
        Message::ChangeColor(_, 20, _) => {
            println!("Change color with green value 20"); // this arm matches
        }
        Message::ChangeColor(_, _, 20) => println!("Change color with blue value 20"),
        _ => println!("Other message"),
    }
}

fn ownership_with_pattern_matching_example() {
    let msg = Message::Write(String::from("Hello, Rust!"));
    match msg {
        Message::Write(text) => {
            // text takes ownership of the String inside Message::Write
            println!("Message text: {}", text);
        }
        _ => println!("Other message"),
    }
    // Note: pattern matching moves ownership of data out of the enum variant, if the data is on the heap.
    // msg is no longer valid here because its ownership has been moved to text.
    // println!("{:?}", msg); // -> compile error
}

fn unwrap_takes_ownership() {
    // Note: unwrap() takes ownership of self.
    let some_value = Some(String::from("Hello"));
    let some_value_ref: &Option<String> = &some_value;
    // let s = some_value_ref.unwrap(); // this causes a compile error
    // Note: unwrap() requires self to be of type Option<T>, not &Option<T>, because it needs to take ownership of the value inside the Option.

    let opt = Some(5);
    let value = opt.unwrap();
    // `opt` is now consumed and can't be used anymore. We can interpret `unwrap` as taking ownership of `opt` to extract the value.
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Each variant can have different types and amounts of associated data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Implementing methods on an enum
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}