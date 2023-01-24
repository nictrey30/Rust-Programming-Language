// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    //   String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.

    // str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.
    //Since the size is unknown, one can only handle it behind a pointer.
    //This means that str most commonly2 appears as &str: a reference to some UTF-8 data, normally called a "string slice" or just a "slice".
    let days: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    for day in 1..=12 {
        let suffix: &str = match day {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!(
            "On the {}{} day of Christmas, my true love sent to me",
            day, suffix
        );
        for num in (0..day).rev() {
            println!("{}", days[num]);
        }
        println!("");
    }
}
