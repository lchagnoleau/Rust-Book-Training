use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please enter an number between 1 and 100:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You entered: {guess}");
}
