use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //the mut specified that the variable is mutable
        //the line above would not be ok with cmp since you would compare string and int so you need to reformat later

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); //the program will crash if err is returned and not return error atm

        //let guess: u32 = guess.trim().parse().expect("Please type a number!"); this one crashes if no number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), //Actually Ordering::xx are just numbers -1, 1, 0 that are matched
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}