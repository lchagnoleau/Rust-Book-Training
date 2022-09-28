use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret generated number is: {}", secret_number);

    loop {
        println!("Please enter an number between 1 and 100:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u8 = match guess
            .trim() // Delete all whitespace, newline etc...
            .parse() { // Convert String to the type of u8
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You entered: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
