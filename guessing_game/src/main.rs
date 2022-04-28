use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // Using a library

    loop { // looping
        println!("Please input your guess."); // Print String

        let mut guess = String::new(); // Mutable variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Get input, expect error message

        let guess: u32 = match guess.trim().parse() { //String manipulation, match flow, Result type
            Ok(num) => num,
            Err(_) => continue, // continue keyword
        };

        println!("You guessed: {}", guess); // formatting

        match guess.cmp(&secret_number) { // comparison, using enums
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break keyword
            }
        }
    }
}
