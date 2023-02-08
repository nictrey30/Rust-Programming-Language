// references - allow you to refer to some value without taking ownership of it
// when functions have references as parameters instead of the actual values, we don't need to return the values in order to give back ownership, because we never had ownership.
// borrowing - having references as parameters in functions. the catch is that you cannot modify them, because they are borrowed. References are immutable by default. we are not allowed to modify something we have a reference to.
fn main() {
    let mut s1 = String::from("hello");
    // the &s1 syntax lets us create a reference that refers to the value of s1, but does not own it. Because it does not own it, the value it points to will not pe dropped when the reference goes out of scope
    let len = calculate_length(&s1);
    println!("The length of {} is: {}", s1, len);
    change(&mut s1);
    println!("the changed string is now: {}", s1);

    // Mutable references - have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope
    {
        let _r1 = &mut s1;
    } // r1 goes out of scope here, so we can make a new reference without a problem
    let _r2 = &mut s1;
    // we also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don't expect the values to suddenly change from under them! However, multiple immutable references are ok because no one who is just reading the data has the ability to affect anyone else's reading of data.

    // a reference's scope starts from where is introduced and continues through the last time that the reference is used.
    // the following code will compile because the last usage of the immutable references occurs before the mutable reference is introduced:
    let r3 = &s1;
    let r4 = &s1;
    println!("{} and {}", r3, r4);
    // r3 and r4 are no longer used after this point

    let r5 = &mut s1; // no problem
    println!("{}", r5);

    // THIS FOLLOWING CODE WONT WORK!
    // let r3 = &s; // no problem
    // let r4 = &s; // no problem
    // let r4 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r3, r4, r5);
}

// define a calculate_length fn that has a ref to an object as a parameter instead of taking ownership of the value
fn calculate_length(s: &String) -> usize {
    s.len()
} // here, s goes out of scope, but because it does not have ownership of what it refers to,nothing happens

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
