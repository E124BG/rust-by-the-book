fn main() {
    //STACK AND HEAP
    //Stack: FIFO architecture where data is "pilled up"
    //Can add data at the top of the stack (push) amd access top element (pop)

    //Heap: Less organised, we request a certain ammount of space and the data gets stored
    //we then obtain a pointer that points to the data in the heap and push this pointer to
    //the stack

    //Using the stack is faster (no search for free space)

    //When code calls a function, values passed into the function and local vars get pushed
    //onto the stack
    //When function is over, those values are popped off the stack

    //Keeping track of data on the heap and minimizing the amount of duplicate on it are
    //parts of ownership addressed problems

    //OWNERSHIP

    //each value has an owner
    //there is only one owner at a timeS
    //when owner is out of scope, value is dropped

    {
                            //s is not valid here because not yet declared
        let _s = "hello";    //s is valid until end of scope

    }                       //s is not valid anymore (out of scope)


    //String and string literals

    //string literals are hardcoded values in our program.
    //Rust has another type, String, this type manages data allocated on the heap and
    //as such is able to store an ammount of text unknown at compile time.

    //String from string litteral

    let mut s = String::from("Hello");

    s.push_str(", world!"); //psuh_str() appends a literal to a String

    println!("{}", s);

    //So Strings can be mutated, not string literals

    {

        let _s0 = String::from("This is a String");


    }//the space taken in the heap by the String is freed when it goes out of scope
        //implicitely, the function "drop" is called upon it

    { let s1 = String::from("hello");//s1 is created in the heap, with a pointer on the stack
        let s2 = s1;                 // s2 copies informations of the pointer, the data in heap is not copied
                                    //to avoid freeing 2 times the same space at the end
                                    //the s1 var is no longer valid, we cannot use it anymore

        //println!("{}",s1);        will produce an error error[E0382]: borrow of moved value: `s1`
    }

    //Create a deep copy: clone method

    {
        let mut s1 = String::from("hello");
        let mut s2 = s1.clone();
        s1.push_str(", world!");

        println!("s1 = {}, s2 = {}",s1,s2);

    }

    //Values on stack can be copied without cloning
    //integer types like u32
    //Boolean types
    //floating-point types like f64
    //character type char
    //tuples that contain the above mentionned types

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    
    //Ownership and functions

    //Passing a value to a function will do exactly as an assignment does (copy or move)

    let s = String::from("hello");  // s comes into scope
    let s1 = s.clone();

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    println!("{}",s1);              //only the clone can be used (we made a deep copy)

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward


{    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.}


}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

a_string  // a_string is returned and moves out to the calling function
}
