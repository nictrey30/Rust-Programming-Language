fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of String
    let word = first_word(&my_string[..]);
    println!("The first word is: {}", word);

    let my_string_literal = "bye world";

    // first_word works on slices of string literals
    // let word = first_word(&my_string_literal[..]);
    // because string literals are string slices already, tis works too:
    let word = first_word(&my_string_literal);
    println!("The first word is: {}", word);
}
fn first_word(s: &str) -> &str {
    let array_bytes = s.as_bytes();
    for (i, &item) in array_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// a string slice is a reference to a part of a String.
// Rather than a referece to the entire String, it's a reference to a portion of the String
