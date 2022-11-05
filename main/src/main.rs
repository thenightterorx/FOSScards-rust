use std::io;
use rand::Rng;

fn main() {
    println!("Guess number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("input guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {guess}");
}
