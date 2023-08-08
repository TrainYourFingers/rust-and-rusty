use std::io;
// use rand::Rng;

fn main() {
    println!("Langar Burja");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please input your guess: ");

    loop {
        // let secret_number = rand::thread_rng().gen_range(1..=6);
        println!("{secret_number}");
    };
}