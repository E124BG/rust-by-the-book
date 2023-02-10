fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //constants

    //constants are immutable, declared in FULL_CAPS_WITH_UNDERSCORES
    //their type must be annotated
    //they can be declared in any scope, including global scope
    //have to be set to a constant expression (not result of a computation done at runtime)

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //shadowing

    
}

// fn error1() {
//  Impossile to assign twice an immutable var
//     let x = 5;
//     println!("The value of x is {x}");
//     x = 6;
//     println!("The value of x is {x}");
// }

