fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    // let mut user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    // user1.email = String::from("anotheremail@example.com");
    // println!("The {} email is: {}", user1.username, user1.email);
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    // struct update syntax ..
    let user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        ..user1
    };
}
