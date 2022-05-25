extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let random_number: u32 = rand::thread_rng().gen_range(1,11);
    println!("random number: {}", random_number);
    println!("Guessing Game!! \n I've thought of a random number from 1 to 10");

    loop {
        println!("Enter a number to make a guess: \n");

        let mut user_guess = String::new(); // note: variables and references are immutable by default
        io::stdin().read_line(&mut user_guess) //pass in a reference (&) to allow the function to have access to the variable
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", user_guess);

        match user_guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it! Well done! :)");
                break;
            }
        }

    }
}
