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
    }else if choice_of_conv == "f" || choice_of_conv == "F"{
        println!("Fahrenheit mode");

    }
    }
}
