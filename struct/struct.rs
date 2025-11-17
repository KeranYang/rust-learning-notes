// Note: To enable the debug trait for a struct, we use the #[derive(Debug)] attribute.
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("keran@mail.com"),
        username: String::from("keran123"),
        active: true,
        sign_in_count: 1,
    };
    // Note: use {:#?} pretty print debug
    println!("User 1 is {:#?}", user1);

    // Note: the entire struct instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user2 = User {
        email: String::from("chuqiao@mail.com"),
        username: String::from("chuqiao456"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("chuqiao2@mail.com");
    println!("User 2 is {:#?}", user2);

    // Note: borrow checker tracks ownership permissions at both the struct level and the field level.
    // If field username is borrowed, then both user2 and field username lose their ownership permissions until the borrow ends.
    // However, other fields of user2, such as email, remain accessible.
    let username_ref = &mut user2.username;
    // println!("name of user2 is {}", user2.username);
    // -> compile error because username is borrowed as mutable
    // println!("user2 is {:#?}", user2);
    // -> compile error because user2.username is borrowed as mutable
    // this line is valid because user2.email is not borrowed
    println!("email of user2 is {:#?}", user2.email);
    username_ref.push_str("_updated");
    println!("user2 is {:#?}", user2);
}