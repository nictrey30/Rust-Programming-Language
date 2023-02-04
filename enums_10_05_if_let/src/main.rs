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
// count all the non-quarter coins while also announcing the sate of quarters
fn main() {
    let coin1 = Coin::Penny;
    // let coin2 = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    // match coin1 {
    //     Coin::Quarter(state) => println!("Stae quarter from {:?}", state),
    //     _ => count += 1,
    // }
    // or
    if let Coin::Quarter(state) = coin1 {
        println!("Stae quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("The count of non-quarter coins is: {}", count);
}
