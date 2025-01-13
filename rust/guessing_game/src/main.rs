use std::io::{self, Write};

fn main() {
    println!("Guess the number!");
    print!("Please input your guess: ");
    io::stdout().flush().unwrap_or_default();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
