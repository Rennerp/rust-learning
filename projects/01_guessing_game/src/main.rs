use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::random_range(1..=100);
    let mut attempts = 0;
    let mut guesses: Vec<u32> = vec![];

    println!("Guess the number from 1 to 100.");
    println!("How many attempts do you want?");

    let max_attempts = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim().parse() {
            Ok(attempts) => break attempts,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
    };

    loop {

        if attempts > max_attempts {
            println!("Max numbers of attempts reached, try again!");
            break;
        }

        println!("Enter a guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // parse returns Result, so match handles both valid and invalid input.
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        guesses.push(guess);
        attempts += 1;

        // cmp returns an Ordering enum: Less, Greater, or Equal.
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too large."),
            Ordering::Equal => {
                println!("Correct in {attempts} attempts.");
                println!("Previous guesses: {guesses:?}");
                break;
            }
        }
    }
}
