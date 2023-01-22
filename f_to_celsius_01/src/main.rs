// convert temp from f to c
use std::io;
fn main() {
    loop {
        let mut temp_fahren = String::new();
        println!("please input the temp in Fahrenheit: ");
        io::stdin()
            .read_line(&mut temp_fahren)
            .expect("failed to read line!");
        let temp_fahren: i32 = match temp_fahren.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };
        println!("The temp in Fahrenheit is: {}", temp_fahren);
        let celsius = (temp_fahren - 32) * 5 / 9;
        println!("The temp in celsius is: {}", celsius);
        let continue_game = is_quitting();
        if continue_game == false {
            break;
        } else {
            continue;
        }
    }
}

fn is_quitting() -> bool {
    loop {
        println!("Do you want to continue? y/n: ");
        let mut quitting = String::new();
        io::stdin()
            .read_line(&mut quitting)
            .expect("failed to read line!");
        let quitting = quitting.trim_end().to_lowercase();
        let quitting_chars: Vec<char> = quitting.chars().collect();
        let quitting_char = quitting_chars[0];
        if quitting_char == 'y' {
            return true;
        } else if quitting_char == 'n' {
            return false;
        } else {
            println!("Invalid input! y/n only ")
        }
    }
}
