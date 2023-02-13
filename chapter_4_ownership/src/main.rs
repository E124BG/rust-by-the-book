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
    //there is only one owner at a time
    //when owner is out of scope, value is dropped

    {
                            //s is not valid here because not yet declared
        let _s = "hello";    //s is valid until end of scope

    }                       //s is not valid anymore (out of scope)


    //String and string literals

    //strings (or string literals) are hardcoded values in our program.
    //Rust has another type, String, this type manages data allocated on the heap and
    //as such is able to store an ammount of text unknown at compile time.

    //String from string litteral

    let mut s = String::from("Hello");

    s.push_str(", world!"); //psuh_str() appends a literal to a String

    println!("{}", s);
}
