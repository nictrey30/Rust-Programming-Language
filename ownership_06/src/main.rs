// Ownership rules
// Each value in Rust has a varible that's called its owner
// There can only be one owner at a time
// When the owner goes out of scope, the value will be dropped

// a scope is a range within a program for wich an item is valid
fn main() {
    // String can be mutated but literals cannot
    let mut s = String::from("hello");
    s.push_str("  world!"); // push_str() appends a literal to a string
                            // println!("{}", s);

    // move
    let s1 = String::from("hello");
    // when we assign s1 to s2, the String data is copied; pointer, length and capacity, that are on the stack. It doesn't copy the data on the heap that the pointer refers to
    // When the variable goes out of scope, Rust calls the drop function, but bot pointers point to the same location in heap. When s1 and s2 go out of scope, they will bth try to free te same memory(known as double free error)
    // To ensure memory safety, Rust considers s1 to no longer be valid and, therefore, Rust doesn't need t free anything when s1 goes out of scope
    let s2 = s1;
    println!("{}, world!", s2);
    // println!("{}, world!", s1); - doesn't work: value borrowed after the move
    // we could say that s1 was moved into s2
    // Rust will never automatically create "deep" cpies of your data

    // clone
    let s3 = String::from("hello");
    // if we do want to deeply copy the heap data, not just the stack data, we can use a common method called clone
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // stack-only data: copy
    // Rust has a special annotation called the Copy trait and if a type has Copy trait, an older variable is still usable after assignment
    // Rust won't let us annotate a type with the Copy trait if the type or any of its parts has implemented the Drop trait
    // types that have Copy trait: all integers types, boolean, floating point, char, tuples(if they only ontain types that are also Copy)
}
