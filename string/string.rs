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
  let _str_literal: &str = "Hello, world!";

  all_about_string();

  size_of_types();

  index_to_strings_is_not_allowed();

  questions_about_string();

  // Note: last but not least, String doesn't allow indexing because it's not clear what an index would mean in the context of UTF-8 encoded strings.
  // Is it a byte index, a character index, or a grapheme cluster index?
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

fn all_about_string() {
  // Two ways of initializing a String
  let _ss1 = String::from("abc");
  let _ss2 = "def".to_string();

  // The push_str method appends a string slice to the String.
  // It does not take ownership of the string slice.
  let mut s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  s1.push_str(&s2);
  println!("can print s1 because s1 still owns the data: {}", s1); // This will print "Hello, world!"
  println!("can print s2 because s2 is not moved: {}", s2); // This will print "world!"

  // Another way of appending is using the + operator.
  // The difference is that the + operator takes ownership of the left operand and returns a new String.
  let s3 = String::from("Goodbye, ");
  let s4 = "world!";
  let s5 = s3 + "world!"; // s3 is moved here and can no longer be used
  // behind the scenes,
  // the + operator uses the add method which takes ownership of the left operand (s3) and borrows the right operand (&str).
  // `fn add(self, s: &str) -> String {}`
  // println!("s3: {}", s3); // This line would cause a compile error
  println!("can print s4 because s4 is a &str and not moved: {}", s4);
  println!("s5: {}", s5);

  // Another way to concatenate strings is using the format! macro.
  let s6 = String::from("tic");
  let s7 = String::from("tac");
  let s8 = String::from("toe");
  let s9 = format!("{}-{}-{}", s6, s7, s8);
  // Note: format! does not take ownership of any of its arguments.
  println!("s9: {}", s9);
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

// Note: indexing into a String is not allowed because it's not clear what an index would mean in the context of UTF-8 encoded strings.
// Instead, use methods like chars() or bytes() to iterate over the characters or bytes of a String.
fn index_to_strings_is_not_allowed() {
  // Let's try initializing a String with emojis
  let s = String::from("😊");
  // We can use chars() to iterate over the characters of the String
  for c in s.chars() {
    println!("{}", c);
  } // 😊
  // Now, if we use bytes(), it's actually 4 bytes because each emoji is represented by 4 bytes in UTF-8 encoding.
  for b in s.bytes() {
    println!("{}", b);
  }
  // 240
  // 159
  // 152
  // 138
  // This is exactly why we don't allow indexing into a String.

  // println!("{}", s[1]); -> compile error - error[E0277]: the type `str` cannot be indexed by `{integer}`
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

  /*
  Question: What is the maximum number of times a heap allocation could occur in this program? Write your answer in digits, e.g. 0 or 1.
  ```
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = s1 + "-" + &s2 + "-" + &s3;
  ```
  Answer: 7.
  The first 3 allocations are from the String::from calls for s1, s2, and s3.
  The next 4 allocations occur during the concatenation process, one for each + operation.
  For each + operation, if the allocated capacity is insufficient to hold the new combined string, a new allocation will be made.
   */
}
