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

    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours");

    //shadowing

    //a var can be shadowed by a second var (it takes its name and any further reference
    //will refer to the second var until it is shadowed itself or the scope ends)
    //we can shadow a var by using the same name following the let keyword

    //we technically create a new var so no need to make it mutable

    let y = 5;

    let y = y + 1;

    {
        //shadowing only in this scope
        let y = y * 2;
        println!("value of y in inner scope: {y}");
    }

    println!("value of y in outer scope: {y}");

    //we can also have a shadowing var with another type
    //i.e. a var that counts the spaces in another var

    // let spaces = "   ";
    // let spaces = spaces.len();
}

// fn error1() {
//  Impossile to assign twice an immutable var
//     let x = 5;
//     println!("The value of x is {x}");
//     x = 6;
//     println!("The value of x is {x}");
// }

