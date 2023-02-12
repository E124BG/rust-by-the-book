use std::io;

fn main() {
    

    let mut count: u32 = 0;
    //let's remember the last 2 elements of the sequence and compute the next one based on them
    //could use an array for this
    let mut fib: u32 = 0;
    let mut fib0: u32 = 0;
    let mut fib1: u32 = 0;


    loop{
        println!("Please enter the index of the fibonacci number you wish to generate: ");
        let mut fibonacci_index = String::new();
        io::stdin().read_line(&mut fibonacci_index).expect("Failure to read line"); //ask for input

        let fibonacci_index: u32 = match fibonacci_index.trim().parse() {//trim and parse input
            Ok(num) => num,
            Err(_) => continue,
        };

        while count <= fibonacci_index{//will only enter while loop if given integer
            if count == 0{//we return fib1, so for index 0 and 1 we give it directly
                fib = 0;
                fib1 = 0;
            }
            else if count == 1{
                fib0 = 1;
                fib1 = 1;
            }
            else {
                fib1 = fib + fib0;//compute current index
                (fib0,fib) = (fib1, fib0);//remember last computed fib and one-before-the-last
            }
            println!{"index: {count}, fibonacci number: {fib1}"};
            count = count + 1;
            
        }
        break;


    }

}
