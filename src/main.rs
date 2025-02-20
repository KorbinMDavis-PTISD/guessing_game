/*!
*  A Simple Guessing Game
* 
*  Depends on the rand Crate
*/

// Bring the Input/Output Library into scope
// The Input/Output Library is part of the Standard Library: std
use std::{cmp::Ordering, io}; // Use the Input/Output Library
use rand::Rng; // Use the Rng Library from Rand

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        io::stdin() // Can also use std::io::stdin() without the use at the top
            .read_line(&mut guess) // Appends the user input from console to guess
                                // & is a reference and &mut is a mutable reference
            .expect("Failed to read line"); // If read_line() returns 'err',
                                            // expect() will catch it


        // Shadow a new guess variable over the old one
        // trim() Removes leading and trailing whitespace
        // parse() parses the string for other data
        // We tell Rust that it's an unsigned 32-bit integer
        // by putting a colon after guess and then adding u32
        // expect() handles errors when parse() recieves non integers
        // Something other than an integer will make parse() return Err

        //.expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

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
