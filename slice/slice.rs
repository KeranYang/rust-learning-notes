// what is a slice in rust?
// A slice in Rust is a **reference** to a contiguous sequence of elements in a collection, such as an array or a vector.
// Slice DOES NOT own the data it references; instead, it provides a view into the data.
fn main() {
    simple_slice_example();
    slices_are_fat_pointers();
}

fn simple_slice_example() {
    let arr = [10, 20, 30, 40, 50];
    // Note: syntax of creating a slice from an array
    let slice = &arr[1..4];
    println!("Slice: {:?}", slice);
    let vec = vec![100, 200, 300, 400, 500];
    // Note: syntax of creating a slice from a vector
    let vec_slice = &vec[2..];
    println!("Vector Slice: {:?}", vec_slice);
    // Note: range syntax: 1. start..end (inclusive start, exclusive end)
    // 2. start.. (from start to the end)
    // 3. ..end (from the beginning to end-1)
}

fn slices_are_fat_pointers() {
    let arr = [1, 2, 3, 4, 5];
    // the syntax of a slice type is &[T], where T is the element type.
    let slice: &[i32] = &arr[0..3];
    // Note: A slice is a fat pointer consisting of:
    // 1. A pointer to the first element of the slice
    // 2. The length of the slice
    let ptr = slice.as_ptr(); // Pointer to the first element
    let len = slice.len();    // Length of the slice
    println!("Slice Pointer: {:p}", ptr);
    println!("Slice Length: {}", len);
}