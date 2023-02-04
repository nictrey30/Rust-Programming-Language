#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    American_Samoa,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    District_of_Columbia,
    Florida,
    Georgia,
    Guam,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Minor_Outlyin_Islands,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    New_Hampshire,
    New_Jersey,
    New_Mexico,
    New_York,
    North_Carolina,
    North_Dakota,
    Northern_Mariana_Islands,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    Puerto_Rico,
    Rhode_Island,
    South_Carolina,
    South_Dakota,
    Tennessee,
    Texas,
    US_Virgin_Islands,
    Utah,
    Vermont,
    Virginia,
    Washington,
    West_Virginia,
    Wisconsin,
    Wyoming,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let coin1 = Coin::Quarter(UsState::Arizona);
    println!("The value of {:?} is {}", coin1, value_in_cents(&coin1));

    // concise control flow with 'if let'
    let some_u8_value = Some(0u8);
    // we want to do something with the Some(3) match, but do nothing with any other Some<u8> value or None. To satisfy match we need to add  _ => ()
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    // or we can use if let, which behaves exactly like match
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // you can think of 'if let' as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values
}
fn value_in_cents(coin: &Coin) -> u8 {
    // matches in Rust are exhaustive, we must exhaust every last possibility in order for the code to be valid
    // The _ pattern will match any value. By putting it after our other arms, the _ will match all the possible cases that aren't specified before it. The () is just the unit value, so nothing will happen in the _ case. As a result, we can say that we want to do nothing for all the possible values that we don't list before the _ placeholder.

    match &coin {
        Coin::Dime => 1,
        Coin::Nickel => 5,
        Coin::Penny => 10,
        // we added a variable 'state' to the pattern that matches values of the variant Coin::Quarter.
        // when it matches, the 'state' variable will bind to the value of that quarter's state
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}
