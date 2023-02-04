// the Option type is an enum defined by the standard library. It encodes the scenario in which a value could be something or it could be nothing
// <T> means the Some variant of the Option enum can hold one piece of data of any type
// if we use None rather than Some, we will need to tell tust what type of Option<T> we have
// let absent_number: Option<i32> = None;

// you have to convert an Option<T> to a T before you can perform T operations on it
// in order to have a value that can possibly be null, you must exlicitly opt in by making the type of that value Option<T>

// in order to use an Option<T> value, you want to have code that will handle each variant
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let coin1 = Coin::Nickel;
    println!(
        "The value in cents of {:?} is: {}",
        coin1,
        value_in_cents(&coin1)
    );
}
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        // the code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire match expression
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
