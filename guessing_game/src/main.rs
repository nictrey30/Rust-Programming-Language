//the Rng trait defines methods that random number generators implement, and this trait must be in scope for us in order to use those methods
use rand::Rng;
use std::io;

// like Result, Ordering is another enum with the variants: less, Greater, Equal
// the cmp method compares two values and it takes a reference to whatever you want to compare with. Then it returns a variant of the Ordering enum we brought into the scope with the use statement
use std::cmp::Ordering;

fn main() {
    let mut num_of_guesses: i32 = 10;
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number);

    loop {
        if num_of_guesses == 0 {
            println!("You lost!");
            break;
        }
        if num_of_guesses == 10 {
            println!("Guess the number! ");
        } else {
            println!("You have {} guesses left!", num_of_guesses);
        }
        println!("Please input your guess.");
        // creates a mutable variable that is currently bound to a new, empty istance of a String
        let mut guess = String::new();
        io::stdin()
            // read_line takes the input and puts it into a string, but it also returns a value, in this case an io::Result
            // The Result types are enums(a type that can have a fixed set of values aka enums variants)
            // if it returns an Err value, expect will cause the program to crash and display the message that you passed as an arg to expect.
            // if it returns an Ok value, expect will take the return value that Ok is holding and return just that value
            .read_line(&mut guess)
            .expect("Failed to read line!");
        // we create a variable named guess. Rust allows us to shadow the previous value of guess with a new one. This is often used in situations in which you want to convert types

        // the call to parse could easily return an error and because of that the parse method returns a Result type
        // if parse returns Err, the expect call will crash the game and print the msg, but if successful, expect will return the number that we want from the Ok value
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // ignoring a non-number guess and continue with another guess, _ is a catchall value
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        num_of_guesses -= 1;

        // the u32 annotation of guess and the comparison with secret_number means that Rust will infer that secret_number should be a u32 as well
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// when you want to update, Cargo will ignore the lock file and figure out all the latest versions that fit the specs in Cargo.toml
