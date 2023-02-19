fn main() {
    exercise_1();

    exercise_2();

    exercise_3();

    exercise_4();

    exercise_5();

    exercise_6();
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

    assert_eq!(u8::MAX, 255);// 0 to 255

    println!("Success!");
}

fn exercise_5() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}",v1,v2);
 }

 fn exercise_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
}
