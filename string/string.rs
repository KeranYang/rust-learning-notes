fn main() {
  // Note: `String::from` returns an owned String, not a reference.
  // Note: String is a heap-allocated data structure, meaning the data (e.g., "Keran") is stored on the heap,
  // while the variable (s) holds the ownership to that data.
  let s = String::from("Keran");
  println!("{}", s);

  // String is not just a pointer on its own.
  // It is a complex data structure that consists of three parts:
    // 1. A pointer to the heap-allocated memory where the actual string data is
    // 2. The length of the string (number of bytes)
    // 3. The capacity of the allocated memory (total bytes allocated)
  // The actual string data is stored on the heap,
  // while the pointer, length, and capacity are stored on the stack.

  string_str_and_slice();

  // Note: string literal is &str type
  // string literals are immutable references since &str is an immutable string slice.
  let str_literal: &str = "Hello, world!";

  size_of_types();

  questions_about_string();
}

fn string_str_and_slice() {
  // Note: String owns the heap data.
  let s = String::from("Hello, world!");
  // Note: &String is a reference to a String
  let string_ref: &String = &s;
  // Note: &str is a string slice, which is a view/reference to a portion of a String
  let str_slice: &str = &s[..];
  // Note: who owns the data?
  // s owns the heap data.
  println!("String: {}", s);
  println!("&String: {}", string_ref);
  println!("&str: {}", str_slice);
}

fn size_of_types() {
  use std::mem::size_of;
  // String - Typically 24 bytes on 64-bit systems, includes pointer, length, capacity.
  println!("Size of String: {} bytes", size_of::<String>());
  // &String - Typically 8 bytes on 64-bit systems, just a pointer to the String structure.
  println!("Size of &String: {} bytes", size_of::<&String>());
  // &str - Typically 16 bytes on 64-bit systems, includes pointer and length.
  println!("Size of &str: {} bytes", size_of::<&str>());
}

fn questions_about_string() {
  /*
  let mut s = String::from("hello");
  for &item in s.as_bytes().iter() {
    if item == b'l' {
      s.push_str(" world");
    }
  }
  println!("{s}");
   */
  // Question: why does the above code not compile?
  // Answer: immutable borrow occurs at s.as_bytes() and lasts until the end of the for loop.
  // During this time, we cannot mutate s by calling s.push_str(" world").

  /*
  Note: &[String] is almost always preferred over &Vec<String> for function parameters.
  Question: why can &[String] accept more types than &Vec<String>?
  Answer: &[String] is a slice type that can represent a view into any contiguous sequence of String elements,
  whether they are stored in an array, a vector, or any other collection that provides a contiguous memory layout.
  On the other hand, &Vec<String> is a reference specifically to a Vec<String> type.
   */

  /*
  Question: when is &str better than String?
  Answer: &str is better than String when:
  1. You want to **avoid heap allocation** for small or temporary strings.
  2. You want to work with string literals or borrowed string data **without taking ownership**.
   */
}
