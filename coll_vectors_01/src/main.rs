// collections that are used very often in Rust:
// vectors - allows you to store a variable number of values next to each other
// strings - a collection of characters
// hash map - allows you to associate a value with a particular key

// vectors can only store values of the same type
// vectors are implemented using generics
fn main() {
    let _v1: Vec<i32> = Vec::new();
    // because we given v2 initial values, Rust can infer the type, Rust provides vec! macro for convenience
    let _v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);

    // reading elements of vectors
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The 3rd elem is: {}", third);
    // or by get method, which gives us back an Option<T>
    match v4.get(2) {
        Some(third) => print!("The third elem is: {}\n", third),
        None => println!("There is no third elem."),
    }
    // attempting to add an element to a vector while holding a reference to an item will not work

    // iterating over values in a vector
    let v5 = vec![100, 22, 53];
    for i in &v5 {
        println!("{}", i);
    }
    // we can also iterate over mutable references in order to make changes to all elements
    let mut v6 = vec![100, 22, 53];
    for i in &mut v6 {
        // to change the value that the mutable reference refers to, we have to use the deference operator (*)
        *i += 10;
    }
    println!("The modified vector v6 is: ");
    for i in &v6 {
        println!("{}", i);
    }

    // using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
}
