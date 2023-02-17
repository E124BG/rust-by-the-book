fn main() {

    exercise_1();

    exercise_2();

    exercise_3();

    exercise_4();

    exercise_5();

    exercise_6();

    exercise_7();




    //based on https://practice.rs/variables.html



}

fn exercise_1(){

    //1.
    // Fix the error below with least amount of modification to the code

    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32 = 0; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");

}

fn exercise_2() {
    //Fill the blanks in the code to make it compile

    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

fn exercise_3(){
    // fix the error below with least ammount of modification

    let x: i32 = 10;
    let y = 0;
    {
        let y: i32  = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);


}

fn exercise_4() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}

fn exercise_5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

fn exercise_6() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    let _x = _x; 

    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!"; 

    println!("Success!");
}

fn exercise_7(){
    //fix the warning (unused var)

    //sol 1 : use the var

    let x = 1;

    println!("The var is {}", x);

    //sol 2 : name the var with underscore

    let _x0 = 1;
}


