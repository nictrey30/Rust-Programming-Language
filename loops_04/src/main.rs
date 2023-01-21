fn main() {
    // returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // when associated with a loop, a break expression may be used to return a value from that loop
            //  If no value is specified, break; returns (). Every break within a loop must return the same type.
            break counter * 2;
        }
        // After the loop, we use a semicolon to end the statement that assigns the value to result.
    };
    println!("The result is {}", result);

    // while loop
}
