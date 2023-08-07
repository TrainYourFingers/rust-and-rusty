use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
 println!("Guess the number!");
 
 let secret_number = rand::thread_rng().gen_range(0..=100);
 
    loop { 
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");


    let guess: u32 = guess.trim().parse().expect("Please input a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("You guessed a little low."),
        Ordering::Greater => println!("You guessed a little high."),
        Ordering::Equal => println!("You guessed it right."),
    }
    }
}