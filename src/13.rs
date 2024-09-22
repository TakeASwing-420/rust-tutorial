#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,  // String does not implement the Copy trait
    email: String,
    sign_in_count: u64, // u64 implements the Copy trait
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"), // New email
        ..user1  // Copy other fields from user1
    };
    user1.sign_in_count += 1;
    println!("{}", user1.sign_in_count);
    println!("{}", user2.sign_in_count); 
    println!("here is the user {user2:?}");
    /*  
    println!("{}", user1.username);
     */
    // ERROR: user1.username has been moved!
}
