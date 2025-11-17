fn main() {
  // Note: `String::from` returns an owned String, not a reference.
  // Note: String is a heap-allocated data structure, meaning the data (e.g., "Keran") is stored on the heap,
  // while the variable (s) holds the ownership to that data.
  let s = String::from("Keran");
  println!("{}", s);
}
