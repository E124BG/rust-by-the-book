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


    println!("Hello, world!");

    {
                            //s is not valid here because not yet declared
        let _s = "hello";    //s is valid until end of scope

    }                       //s is not valid anymore (out of scope)
}
