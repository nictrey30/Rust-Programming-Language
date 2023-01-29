// String Slices - a string slice is a reference to part of a String

// write a function that takes a string and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

// let s = String::from("hello world");
// rather than a reference to the entire String, it's a reference to a portion of the String
// internally, the slice data structure stores the starting position and the length of the slice
// let hello = &s[0..5];
// let world = &s[6..11];

fn main() {
    let s = String::from("hello world");
    //now when we call first_word, we get back a single value that is tied to the underlying data.
    let word = first_word(&s);
    // this will throw an error
    // s.clear();
    // if we use s.clear(); error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. Rust disallows this, and compilation fails.
    println!("The first word is: {}", word);
}

// fn first_word returns a slice. The type that signifies "string slice" is written as "&str"
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead. The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// String Literals Are Slices
// let s = "Hello, world!";
// The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.
