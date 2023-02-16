fn main() {

    exercise_1();

    exercise_2();


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
}
