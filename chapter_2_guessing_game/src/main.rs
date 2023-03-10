use rand::Rng;
use std::cmp::Ordering;
use std::io;
//documentation of crates accessed with $cargo doc --open

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //the max is inclusive here

    println!("The secret number: {secret_number}");
    //comment

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // mut means that the var "guess" is mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //& means that here its a reference to the mutable var "guess"
        // references can be used without having to copy the entire var in memory

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //trim removes any whithespace, parse converts string to number
        //if parse fails because guess is not a number, then try again

        println!("You guessed {guess}");

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
