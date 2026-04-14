use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
    let correct_guess: u32 = rng.gen_range(0..100);
    println!("Guess a number between 0 and 100");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&correct_guess) {
            Ordering::Less => println!("too small"),// too small,
            Ordering::Greater => println!("too big"), // too big,
            Ordering::Equal => {
            	println!("You guessed it!");
             	break;
             }// correct, break out of loop,
        }
    }
}