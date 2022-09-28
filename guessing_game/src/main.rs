use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret generated number is: {}", secret_number);

    println!("Please enter an number between 1 and 100:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You entered: {guess}");
}
