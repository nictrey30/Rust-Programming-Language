// references - allow you to refer to some value without taking ownership of it
// when functions have references as parameters instead of the actual values, we don't need to return the values in order to give back ownership, because we never had ownership.
// borrowing - having references as parameters in functions. the catch is that you cannot modify them, because they are borrowed. References are immutable by default. we are not allowed to modify something we have a reference to.
fn main() {
    let s1 = String::from("hello");
    // the &s1 syntax lets us create a reference that refers to the value of s1, but does not own it. Because it does not own it, the value it points to will not pe drpped when the reference goes out of scope
    let len = calculate_length(&s1);
    println!("The length of {} is: {}", s1, len);
}

// define a calculate_length fn that has a ref to an object as a parameter instead of taking ownership of the value
fn calculate_length(s: &String) -> usize {
    s.len()
} // here, s goes out of scope, but because it does not have ownership of what it refers to,nothing happens
