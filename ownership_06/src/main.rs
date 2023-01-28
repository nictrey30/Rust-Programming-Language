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
    let s1_1 = String::from("hello");
    // if we do want to deeply copy the heap data, not just the stack data, we can use a common method called clone
    let s2_1 = s1_1.clone();
    println!("s1_1 = {}, s2_1= {}", s1_1, s2_1);

    // stack-only data: copy
    // Rust has a special annotation called the Copy trait and if a type has Copy trait, an older variable is still usable after assignment
    // Rust won't let us annotate a type with the Copy trait if the type or any of its parts has implemented the Drop trait
    // types that have Copy trait: all integers types, boolean, floating point, char, tuples(if they only ontain types that are also Copy)

    // Ownership and Functions
    // passing a variable to a function will move or copy, just as assignment does

    let s1_2 = String::from("some_string"); // s5 comes into scope
    takes_ownership(s1_2); // s5's value moves into the function, and it is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would into the function, but i32 is Copy, so it's ok to still use x afterwards

    // Return Values and Scope
    // return values can also transfer ownership
    let s1_2 = gives_ownership(); // gives_ownership moves its return value into s1_2

    let s2_2 = String::from("some_string - gives ownership"); // s2_2 comes into scope

    // s2_2 is moved into takes_and_give_back, which also move its return value into s3_2
    let s3_2 = takes_and_give_back(s2_2);

    // what if we want to let a function use a value, but not take ownership? It is annoying that anything we pass it also needs to be passed back if we want to use it again
    // returning multiple values using a tuple
    let s1_3 = String::from("returning_multiple_values");
    let (s2_3, len) = calculate_length(s1_3);
    println!("The length of '{}' is {}", s2_3, len);
} // here s3_2 goes out of scope and is dropped. s2_2 goes out of scope, but was moved into takes_and_gives_back, so nothing happens. s1_2 goes out of scope and is dropped

// some_string comes into scope
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // here, some_string goes out of scope and 'drop' is called

// some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // here, some_integer goes out of scope. Nothing special happens

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("some_string - gives ownership");
    // some_string is returned and moves out to the calling function
    some_string
}

// takes_and_give_back will take a String and return one
fn takes_and_give_back(a_string: String) -> String {
    // a_string is returned and moves out to the calling function
    a_string
}

// The ownership of avariable follows the same pattern every time: assigning a value to another moves it.
// When a variable that includes data on the heap goes out of scope, the value will be cleaned up by 'drop' unless the data has been moved to be owned by another variable
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
