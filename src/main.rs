/*!
*  A Simple Guessing Game
* 
*  Depends on the rand Crate
*/

// Bring the Input/Output Library into scope
// The Input/Output Library is part of the Standard Library: std
use std::io; // Use the Input/Output Library
use rand::Rng; // Use the Rng Library from Rand

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    io::stdin() // Can also use std::io::stdin() without the use at the top
        .read_line(&mut guess) // Appends the user input from console to guess
                               // & is a reference and &mut is a mutable reference
        .expect("Failed to read line"); // If read_line() returns 'err',
                                        // expect() will catch it

    println!("You guessed: {}", guess);
}
