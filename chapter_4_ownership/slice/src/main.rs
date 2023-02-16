fn main() {

    let hello: String = String::from("Hello world!");

    println!("{}",hello);

    //function without string slice

    first_word(&hello);


}

fn first_word(s: &String) -> usize {//Takes a String ref in, gives an index back
    let bytes = s.as_bytes();//let's convert the string into bytes

    println!("{:?}",bytes);

    for (i, &item) in bytes.iter().enumerate() {//let's iterate over the array obtained
        if item == b' ' {//we search for the byte literal syntax of ' '
            return i;//return current index if we find a space
        }
    }

    s.len()//return length of the String if it contains no space (whole String is one word)
}
