fn main() {
    //how to use calculate_length without giving ownership of the value
    //we use a reference instead of a pointer
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //references have 2 major constraints to avoid race conditions

    //1.We can mutate only one reference at a time
    //2.We can't borrow a reference that is being mutated

    //     let mut s = String::from("hello");

    //     let r1 = &mut s;
    //     let r2 = &mut s;

    //     println!("{}, {}", r1, r2);
    //      won't compute because 2 mutable refs at the same time
}

fn calculate_length(s: &String) -> usize {
    //reference &String refers to the value without taking ownership
    s.len()
} //s is not dropped (its owner is still in scope)
