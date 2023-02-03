struct User {
    // used String instead of &str, because we want the instances of this struct to own all of its data and for that data to be valid as long as the entire struct is valid

    // it is possible for structs to store references to data owned by someone else, but to do so require the use of 'lifetimes'
    // 'lifetimes' ensure that the data referenced by a struct is valid for as long as the struct is.
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    // let mut user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    // user1.email = String::from("anotheremail@example.com");
    // println!("The {} email is: {}", user1.username, user1.email);
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    // struct update syntax ..
    let _user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        ..user1
    };

    // tuple structs - have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the types of fields.
    // tuple structs are useful when you want to give the whole tuple a name and make the tuple be a diffrent type from other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    // black and origin are diffrent types, because they're instances of diffrent tuple structs
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
