use std::io;
use rand::Rng;
//documentation of crates accessed with $cargo doc --open

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //the max is inclusive here

    println!("The secret number: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
    // mut means that the var "guess" is mutable

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    //& means that here its a reference to the mutable var "guess"
    // references can be used without having to copy the entire var in memory

    println!("You guessed {guess}");
}
