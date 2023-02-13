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
    println!("Hello, world!");
}
