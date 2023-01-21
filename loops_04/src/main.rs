use core::num;

fn main() {
    // because if is an expression, we can use it to the right side of the let statement
    let condition = true;
    // blocks of code evaluate to the last expression in them, so the values that have the potential to be results from each arm of the if MUST be the same type
    let number = if condition { 5 } else { 6 };
    println!("number is: {}", number);
    // returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        // if is an expression
        if counter == 10 {
            // when associated with a loop, a break expression may be used to return a value from that loop
            //  If no value is specified, break; returns (). Every break within a loop must return the same type.
            // you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it
            break counter * 2;
        }
        // After the loop, we use a semicolon to end the statement that assigns the value to result.
    };
    println!("The result is {}", result);

    // while loop
    // while a condition holds true, the code runs; otherwise it exits the loop
    let mut number_2 = 3;
    while number_2 != 0 {
        println!("{}", number_2);
        number_2 -= 1;
    }
    println!("LIFTOFF!!!");

    for number in (1..5).rev() {
        println!("{}!", number)
    }
    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];
    for elem in a.iter() {
        println!("the value is: {}", elem);
    }
}
