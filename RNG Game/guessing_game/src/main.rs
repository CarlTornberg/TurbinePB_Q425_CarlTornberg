use std::io::stdin;

use rand::{random, Rng};

fn main() {

    let mut input = String::new();
    println!("The computer is holding you hostage and you need to guess the number the computer is guessing in order to continue using your computer./n
        Enter a number between 1 and 100. Once you have guessed correctly you're free!");

    loop {
        input.clear();

        println!("Make your guess:");
        if let Err(e) = stdin().read_line(&mut input) {
            println!("Could not read data: {}", e);
            continue;
        }

        if let Ok(guess) = input.trim().parse::<i64>() {
            if !(1..101).contains(&guess) {
                println!("The number is not within the range. Try again.");
                continue;
            }
            let rn: i64 = rand::rng().random_range(1..101);
             
            if rn == guess {
                println!("Wow.. well done.. you both guessed {}. You're finally free!", guess);
                break;
            }
            else if guess < rn {
                println!("Your guess {} is below the computers number {}. Keep trying!", guess, rn);
            }
            else {
                println!("Your guess {} is above the computers number {}. Keep trying!", guess, rn);
            }
        }
        else {
            println!("Please enter a valid number between 1 and 100!");
        }

    }
}
