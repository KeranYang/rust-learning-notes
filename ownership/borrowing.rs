fn main() {
    reference_does_not_take_ownership();
    borrow_checker();
}

fn reference_does_not_take_ownership() {
    let vec = vec![1, 2, 3, 4, 5];
    let first = get_first_int(&vec);
    println!("{}, {}", vec[0], first);
    // Note: vec is still valid here because we passed a reference to it,
    // so ownership was not moved.
}

fn get_first_int(vr: &Vec<i32>) -> i32 {
    vr[0]
}

fn borrow_checker() {
    // Note: case 1: borrow checker ALWAYS fail compilation when there is undefined behavior
    let b1 = Box::new(10);
    let val = extract(&b1); // -> compile error

    // Note: case 2: borrow checker SOMETIMES fail compilation even when there is no undefined behavior
    let mut s1 = String::from("hello");
    let mut s2 = String::from("world");
    transfer_string(&mut (s1, s2)); // -> compile error

    let str_vec = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ];
    reverse(&mut str_vec); // -> compile error
}

fn extract(b: &Box<i32>) -> i32 {
    // this line of code is trying to move the ownership of the Box out of the reference,
    // which would lead to a double free error when both b and b2 go out of scope.
    let b2: Box<i32> = *b;
    *b2
}

fn transfer_string(strs: &mut (String, String)) {
    let fst = get_first(strs); // first mutable borrow occurs here
    let snd = get_second(strs);// second mutable borrow occurs here
    fst.push_str(snd); // first borrow later used here
    snd.clear();
    // although there is no undefined behavior here,
    // borrow checker still fails compilation because
    // he doesn't analyze the code deeply enough to see that get_first and get_second
    // are borrowing different parts of the tuple.
}

fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        std::mem::swap(&mut v[i], &mut v[n - i - 1]); // cannot borrow `*v` as mutable more than once at a time.
        // although there is no undefined behavior here,
        // borrow checker still fails compilation because
        // he doesn't analyze the code deeply enough to see that v[i] and v[n - i - 1]
        // are borrowing different elements of the vector.
    }
}

fn get_first(strs: &mut (String, String)) -> &mut String {
    &mut strs.0
}

fn get_second(strs: &mut (String, String)) -> &mut String {
    &mut strs.1
}