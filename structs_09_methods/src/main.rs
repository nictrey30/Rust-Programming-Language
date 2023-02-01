#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// define a function within context of Rectangle, we start an impl (implementation) block
// methods are defined within the context of a struct, enum or trait object, and their first parameter is always &self, which rerpesents the instance of the struct the method is being called on.

impl Rectangle {
    // Rust knows the type of self is rectangle due to this method's being inside the impl Rectangle context

    // we've chosen &self here because we don't want to take ownership, and we just want to read the data in the struct, notwrite to it.
    // if we wanted to change the instance that we've called the method on as part of what the method does, we'd use &mut self as the first parameter
    // using self as param is rare, use case: prevent the caller from using the original instance, as the method transforms self into something else
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is: {}", rect1.area());
}
