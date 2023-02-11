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

    let _x = 2.03;

    let _y : f32 = 2.0;

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

    let _t = true;

    let _f : bool = false; //with explicit type annotation


    //Character type

    let _c = 'z';
    let _z : char = 'Z'; //with explicit type annotation
    let heart_eyed_cat = '😻'; //as long as we have a unicode char, we can use it

    println!("Cat with heart-eyes (unicode char): {heart_eyed_cat}");

    //Compound types
    //are groups of multiple values

    //tuple type
    //each element has a type and they don't have to be the same
    //tuples have a fixed length, once declared we can't change it

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //destructuration of a tuple

    let (_five_hundred, _six_point_four, _one) = tup;

    //direct access

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    //empty tuple is called "unit" ()
    //expressions implicitely return the unit value if they don't return anything else

    //array type
    //every element in an array has the same type
    //they have a fixed length

    let arr = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    //useful when we want to use stack rather than heap
    //or when we want to ensure we always have the same number of elements

    let _arr_full_of_threes = [3; 5];
    let third_month = arr[2];

    println!("Third month of the year: {third_month}");









}
