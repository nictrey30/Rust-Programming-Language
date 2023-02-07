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
}
