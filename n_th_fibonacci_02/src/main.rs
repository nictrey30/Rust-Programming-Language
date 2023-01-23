// generate the nth fibonacci number
use std::io;

fn main() {
    let num = get_input();
    display(calc_fibonacci, num);
}
fn get_input() -> i32 {
    loop {
        println!("Please input which number from the Fibonacci sequence you want? >= 1");
        let mut nth_fibo_no = String::new();
        io::stdin()
            .read_line(&mut nth_fibo_no)
            .expect("Failed to read line!");
        let nth_fibo_no: i32 = match nth_fibo_no.trim().parse() {
            Ok(num) => {
                if num < 1 {
                    println!("Input a number >= 1");
                    continue;
                };
                num
            }
            // ignoring a non-number guess and continue with another guess, _ is a catchall value
            Err(_) => continue,
        };
        return nth_fibo_no;
    }
}
fn calc_fibonacci(nth: i32) -> i32 {
    let mut prev = 0;
    let mut current = 1;
    for _ in 1..nth {
        let next = prev + current;
        prev = current;
        current = next;
    }
    current
}
fn display(f: fn(i32) -> i32, num: i32) {
    if num == 1 {
        println!("The 1st number is: {}", f(num));
    } else if num == 2 {
        println!("The 2nd number is: {}", f(num));
    } else if num == 3 {
        println!("The 3rd fibonacci number is: {}", f(num));
    } else {
        println!("The {}th fibonacci number is: {}", num, f(num));
    }
}
