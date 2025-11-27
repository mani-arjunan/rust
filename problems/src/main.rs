// write a function that takes a string of words separated by spaces and returns
// the first word it finds in that string. 
// If the function doesn’t find a space in the string, the whole string must be one word,
// so the entire string should be returned.

fn first_word(words: &str) -> u16 {
    let bytes = words.as_bytes();

    for i in 0..bytes.len() {
        if bytes[i] == b' ' {
            return 1;
            // return "Hello asda";
        }
    }

    // return "Manikandan";
    return 2;
}

fn main() {
    let mut str = String::from("Hello HiHas");
    // let mut str1: &str = "Manikandan";
    //
    // str1 = "asdas"

    let word = first_word(&str); // immutable reference
    //
    str.clear(); // mutable reference

    println!("{}", word); // since immutable reference is used, compiler is throwing error
}
