use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = 42;
    let mut attempts = 0;

    println!("Guess the number from 1 to 100.");

    loop {
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

        attempts += 1;

        // cmp returns an Ordering enum: Less, Greater, or Equal.
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too large."),
            Ordering::Equal => {
                println!("Correct in {attempts} attempts.");
                break;
            }
        }
    }
}
