fn main() {

    let hello_string: String = String::from("Hello world");

    println!("{}",hello_string);

    //function without string slice

    first_word(&hello_string);

    //string slices are references to parts of a String

    let hello = &hello_string[0..5];
    //same as &hello_string[..5] (will go from String's begining)
    let world = &hello_string[6..11];
    //same as &hello_string[6..] (will go until end of String)
    
    let word = first_word(&hello_string[..]);

    println!("The first word of the String is: {}", word);

    let word0 = second_word(&hello_string);

    println!("The second word of the String is: {}", word0);


}

fn first_word(s: &str) -> &str {//Takes a String ref in, gives an index back
    let bytes = s.as_bytes();//let's convert the string into bytes

    println!("{:?}",bytes);

    for (i, &item) in bytes.iter().enumerate() {//let's iterate over the array obtained
        if item == b' ' {//we search for the byte literal syntax of ' '
            // return i;//return current index if we find a space
            return &s[0..i];//instead let's return a slice from beginning to first space
        }
    }

    //s.len()//return length of the String if it contains no space (whole String is one word)
    &s[..] //return slice of entire String if no space

}

fn second_word(s: &String) -> &str {

    let bytes = s.as_bytes();
    let mut begining = 0;
    let mut count = 0;
    let mut end = 0;

    for (i, &item) in bytes.iter().enumerate() {

        if item == b' ' {
            if count == 0{
            count += 1;
            begining = i;}
            if count == 1{
                end = i;
                return &s[begining..end];
            }



        }
    }
    &s[begining..]
}
