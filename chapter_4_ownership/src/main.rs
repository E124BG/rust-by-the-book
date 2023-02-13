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
}
