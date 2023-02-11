fn main() {
    println!("Hello, world!");

    another_function(5);

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    //functions can use arguments that are passed
    //arguments types must be declared in RUST
    println!("The value of x is: {x}");
}


//we specify return type of the function as i32
//the returned value is after keyword "return", or simply the last value of the block
fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }
//wont compile because doesnt return a value

//var assignments can be made with values, but not statement
//let x = (let y = 6);
//wont compile
