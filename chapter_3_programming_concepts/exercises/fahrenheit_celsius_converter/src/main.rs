use std::io;

fn main() {

    
    'outer_loop: loop{
        println!("Please choose if you want to convert:\nCelsius to Farhenheit (type C)\nFarenheit to Celsius (type F)");
        let mut choice_of_conv = String::new();
        io::stdin().read_line(&mut choice_of_conv).expect("Failure to read line");

        let choice_of_conv: String = match choice_of_conv.trim().parse() {
            Ok(string) => string,
            Err(_)=> continue,
        };



        if choice_of_conv == "c" || choice_of_conv == "C"{

            println!("Please enter a temperature in Celsius.");

            let mut celsius_input = String::new();
            io::stdin().read_line(&mut celsius_input).expect("Failure to read line");

            

            let celsius_input: f64 = match celsius_input.trim().parse() {
                Ok(num) => num,
                Err(_)=> continue,
            };

            println!("Celsius: {celsius_input}");
            
            let mut fahrenheit: f64 = celsius_input * (9.0/5.0) + 32.0;

            println!("Fahrenheit: {fahrenheit}");
    }else if choice_of_conv == "f" || choice_of_conv == "F"{
        println!("Please enter a temperature in Fahrenheit.");

            let mut fahrenheit_input = String::new();
            io::stdin().read_line(&mut fahrenheit_input).expect("Failure to read line");

            

            let fahrenheit_input: f64 = match fahrenheit_input.trim().parse() {
                Ok(num) => num,
                Err(_)=> continue,
            };

            println!("Fahrenheit: {fahrenheit_input}");
            
            let mut celsius: f64 = ((fahrenheit_input - 32.0) * 5.0)/9.0 ;

            println!("Celsius: {celsius}");

    }
    }
}
