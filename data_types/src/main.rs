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
}