// methods with more parameters
// we want an istance of Rectangle to take another instance of Rectangle
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
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("The area of rect1 is: {}", rect1.area());
    // can_hold passes &rect2, whicih is an immutable borrow to rect2, an istance of Rectangle.
    // This makes sense because we only need to read rect2(rather than write), and we want main to retain ownership of rect2, so we can use it again after calling can_hold method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rectangle::square(30);
    println!("A square: {:?}", sq);
}
// Associated functions - within impl block we are allowed to define functions that don't take self as a parameter.
// associated functions are often used for constructors that will return a new instance of the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // associated function
    // to call this function we use :: syntax
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
