// calculate the area of a rectangle with Structs

// functionality to print out debugging info, but we have to explicitly opt in
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectagle is: {}", calculate_area(&rect1));
    // println!("rect1 is {}", rect1); - doesn't work
    println!("rect1 is {:#?}", rect1);
}
// function defined with one parameter(type Rectangle), whose type is an immutable borrow of a struct Rectangle instance. We want to borrow the struct rather than take ownership of it.
// This way, main retains its ownership and can continue sing rect1
fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
