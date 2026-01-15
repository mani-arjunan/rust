fn main() {
    let mut hello = String::new();
    let world = "world"; // string slice &str
    hello.push_str("Hello"); // u can push new strings into a String using push_str
    hello.push(' '); // u can also push single character using push method
    hello.push_str(world); // u can also push string slices(&str)
    hello.push('!'); // u can also push single character using push method


    // let first_char = hello[0]; //this is basically not possible in rust, unlike other langugage
    // coz rust supports utf8 encoding characters, in our above example hello[0] can
    // print naturally H, but consider different language, where a single letter could take 
    // multiple bytes, in those scenarios the result would be very different.

    // for EG
    // let russian_hello = String::from("Здравствуйте");
    // let first_char = russian_hello[0];
    // in the above example russian_hello[0] is not going to be 3, it could be something else
    // infact the length of this string stored in rust underlying is also 24 and not 12, coz
    // of how utf-8 encodes each unicode into bytes, in order to resolve this
    // u can use [String::].chars() function where u will get each characters, u can also
    // use [String::].bytes() to get the raw bytes

    for c in hello.chars() {
        print!("{}", c);
    }

    println!("{}", hello);
    // NOTE how this works in other languages
    // Python
    // python first reads all the characters in the string and takes the largest unicode byte
    // character and make the character byte length as their individual element length for the
    // array for EG: "mani😀", here actual byte of this string is 8 bytes m-1, a-1, n-1, i-1, 😀-4,
    // python takes the largest character unicode 4byte which is the smiley and then assigns the
    // same across this array something like below
    // [4byte, 4byte, 4byte, 4byte, 4byte]
    // why -> easy to iterate and then length is also same and whenever u do [0] | [1] etc u will
    // get the exact character
    // Javascript
    // js by default takes 2bytes to store every character in a string. for the above example
    // it stores like [2byte, 2byte, 2byte, 2byte, 2byte, 2byte]
    // if u notice our array length is of 6, its coz to store smiley u need extra 2byte, so it
    // took another character in our array, in js for this example when u do arr[4] u will get some
    // random unicode instead of smiley, but in python u would get the smiley.
    // Python tradesoff the space, js tradesoff smileys(lol)
}
