use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new(); //the mut specified that the variable is mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line."); //the program will crash if err is returned and not return error atm

    println!("You guessed: {guess}");
}