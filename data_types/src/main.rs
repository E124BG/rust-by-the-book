fn main() {
    //Rust is statically typed, that means it must know each var type at compile time.
    
    //Scalar types

    //They represent a single value : integers, floating-point numbers, booleans, characters

    //Integer type

    //signed or unsigned : i, u
    //length : 8, 16, 32, 64, 128
    //example : i8 : signed 8-bit
    //signed numbers use two complement's representation, that means that
    //in can store from -(2^n-1) to 2^(n-1)-1 inclusive
    //un can store from 0 to 2^(n)-1 inclusive

    let big_negative_number : i32 = -100_000;

    println!("A big negative number : {big_negative_number}");
    
    //We can also use _ in numerals to make them easier to read

    let big_number : u32 = 100_000;

    println!("A big number : {big_number}");

    //Floating point numbers type

    //floating points are signed, default if f64
    //represented as IEEE-754 standard
    //f64 is double precision
    //f32 is single precision

    let x = 2.03;

    let y : f32 = 2.0;

    //numeric operations

    // addition
    let sum = 5 + 10;

    println!("This is 5 + 10: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("This is 95.5 - 4.3: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("This is 4 * 30: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("This is 56.7 / 32.2: {quotient}");

    // remainder
    let remainder = 43 % 5;
    println!("This is 43 % 5: {remainder}");

    //Boolean type

    let t = true;

    let f : bool = false; //with explicit type annotation


}
