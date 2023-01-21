fn main() {
    another_function(5, 7);

    let x = five();
    let z = plus_one(5);

    let y = {
        let x = 3;
        // expressions do not include ending semicolons. if you add a semicolon at the end of an expression, you turn it into a statement, which will then not return a value
        // expressions do not incude semicolons
        // if you add a semicolon to the end of an expression, you turn it into a statement, which will then NOT return a value
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
// function bodies are made up of a series of statements optionally ending in an expression
// statements are instructions that perform some action and do not return a value.
// expressions evaluate to a resulting value
fn another_function(a: i32, b: i32) {
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

// function with return values
// in Rust, the return value of the function is synonymous with the value of the final expression in the block of the body pf a function.
// you can return early from a function using the return keyword and specifying a value
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
