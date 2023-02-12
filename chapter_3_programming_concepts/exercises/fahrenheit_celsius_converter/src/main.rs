use std::io;

fn main() {

    println!("Please choose if you want to convert:\nCelsius to Farhenheit (type C)\nFarenheit to Celsius (type F)");
    let mut choice_of_conv = String::new();
    io::stdin().read_line(&mut choice_of_conv).expect("Failure to read line");

    loop{
        println!("Please enter a temperature in Celsius.");

        let mut celsius_input = String::new();
        io::stdin().read_line(&mut celsius_input).expect("Failure to read line");

        

        let celsius_input: i32 = match celsius_input.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };

        println!("Celsius: {celsius_input}");
        
        let mut fahrenheit: i32 = celsius_input * 9/2 + 32;

        println!("Fahrenheit: {fahrenheit}");




    }
}
