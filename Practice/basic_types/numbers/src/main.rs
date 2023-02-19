fn main() {
    exercise_1();

    exercise_2();

    exercise_3();

    exercise_4();
}

fn exercise_1() {
    // Remove something to make it work
    let x: i32 = 5;
    let mut _y = 5;

    _y = x;

    let _z = 10; // Type of z ?

    println!("Success!");
}

fn exercise_2() {
    // Fill the blank
    let v: u16 = 32_u8 as u16;

    println!("{}", v);

    println!("Success!");
}

fn exercise_3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn exercise_4 () {
    assert_eq!(i8::MAX, 127);// -128 to 127
    println!("Success! 1 ");

    assert_eq!(u8::MAX, 255);// 0 to 255

    println!("Success! 2 ");
}
