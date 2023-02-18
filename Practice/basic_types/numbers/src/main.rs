fn main() {
    exercise_1();

    exercise_2();

    exercise_3();
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

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
