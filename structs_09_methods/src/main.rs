#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// define a function within context of Rectangle, we start an impl (implementation) block
impl Rectangle {
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
