// WHAT?
// lifetimes are a type of generic parameter(similar to generics like T in Option<T>)
// that specify how long references are valid.
// Every reference in Rust has a lifetime.
//
// WHY?
// the main purpose of lifetimes is to prevent dangling references.
//
// WHEN?
// we define lifetimes when writing function signatures that involve references,
// to instruct the borrow checker about the relationships between the lifetimes of the references.
// we never change lifetimes,
// we only annotate them to help the compiler understand the relationships between references.
//
// Note:
// The borrow checker works by comparing scopes to determine whether all references are valid,
// meaning that they do not outlive the data they point to.
// The comparison is done based on lifetimes.

/*
Why the following code does not compile?
```
fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```
Answer: It returns a reference to data owned by one of the input parameters,
but the compiler cannot determine which one, so it cannot execute the borrow checking correctly.
 */

// Solution: a function signature to express that the returned reference will be valid as long as both input references are valid.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Note: key points about the code above:
// 1. both input s1 and s2 live AT LEAST as long as lifetime 'a.
// 2. the returned reference, the lifetime is the same as the smaller of the two input lifetimes.
//
// Rule: when returning a reference from a function, the lifetime parameter of the return type
// needs to match one of the input lifetime parameters.

fn main() {
    lifetimes_in_functions();
    lifetimes_in_structs();
    static_lifetimes();
    lifetimes_when_casting_between_types();
}

fn lifetimes_in_functions() {
    let string1 = String::from("longest string");
    let string2 = "short";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
}

// Note: here the lifetime annotation means that the ImportantExcerpt struct
// cannot outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetimes_in_structs() {
    // novel owns the String data
    let novel = String::from("Call me Ishmael. Some years ago...");
    // first_sentence is a &str slice that references part of novel
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // ImportantExcerpt cannot outlive novel because it holds a reference to its data
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {}", i.part);
}

// rules of lifetime elision:
// 1. each parameter that is a reference gets its own lifetime parameter.
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self,
//    the lifetime of self is assigned to all output lifetime parameters.
// These rules allow the compiler to infer lifetimes in many common scenarios without explicit annotations.
impl<'a> ImportantExcerpt<'a> {
    // here, following rule1, the two input parameters get their own lifetimes,
    // now, because we have 2 different input lifetimes, following rule3, the output lifetime is assigned to the lifetime of &self.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn static_lifetimes() {
    // 'static lifetime means that the reference is valid for the entire duration of the program.
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

fn lifetimes_when_casting_between_types() {
    use std::fmt::Display;
    /*
    fn add_displayable<T: Display>(
        v: &mut Vec<Box<dyn Display>>,
        t: T
    ) {
        v.push(Box::new(t));
    }
    This function does not compile because the compiler cannot determine the lifetime of t and the Box<dyn Display> created from it.
    Rust requires that the trait object dyn Display must outlive the vector v,
    however the lifetime of T is not specified, so it may not live long enough.

    Marking T as 'static solves the compilation error but is too restrictive, as it requires that T must live for the entire duration of the program.

    Instead, we can introduce a lifetime parameter 'a to express that T must live at least as long as the trait object.
    */
    fn add_displayable<'a, T: Display + 'a>(
        v: &mut Vec<Box<dyn Display + 'a>>,
        t: T
    ) {
        v.push(Box::new(t));
    }
}