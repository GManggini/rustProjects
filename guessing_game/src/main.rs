use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Please input a valid guess");

    println!("You guessed: {guess}");
}
