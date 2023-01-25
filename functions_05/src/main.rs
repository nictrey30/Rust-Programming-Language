// statements - instructions that perform some action and DO NOT return a value
// expressions evaluate to a resulting value
// if you add a semicolon at the end of an expression, you turn it into a statement, which will then not return a value
fn main() {
    another_function(2, 3);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
// functions can return values to the code that calls them
// the return value of the function is synonymous with the value of the final expression in the block of the body of a function
// you can return early from a function by using return and specifying a value
fn plus_one(x: i32) -> i32 {
    x + 1
}
