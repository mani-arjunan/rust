fn main() {
    // let name = "Manikandan";
    // println!("Name => {name}");
    // name = "Manikandan Arjunan"; // this is not possible in rust by default, since it is immutable, we need
    // to add mut in line 2 variable declaration

    // let mut name = "Manikandan";
    //
    // println!("Name initialize => {name}");
    // name = "Manikandan Arjunan"; // this is not possible in rust by default, since it is immutable, we need
    // println!("Name after update => {name}");
    //
    // let age = 12;
    //
    // println!("Before shadowing: {age}");
    // let age = "Manikandan"; //invalid i know, but this is shadowing, technically
    // // i can declare same variable again either with same type or different
    // println!("After shadowing: {age}");

    // datatypes
    // explicit typing, since we are converting from one to another.
    // let age: u16 = "27".parse().expect("Not a age"); 
    //
    // // implicit typing, rust compiler will know the type name on compile time.
    // let name = "Manikandan Arjunan";
    //
    // println!("{age} {name}");
    //
    // tuple
    //
    // let tup1: (i16, char, bool) = (2, 'M', false);
    //
    // let (x, y, z) = tup1; // kinda destructuring in compare with js world.
    //
    // println!("{x} {y} {z}");
    // println!("{}", tup1.0); // this is also possible
    // arrays
    // let arr: [i16; 5] = [1, 2, 3, 5, 6]; // this is one way of declaring array [i16-> type; 5 ->
    // // size]
    // let arr = [1, 2, 3, 5, 6]; // this is another way of declaring array, where the size
    // // and type are determined by the compiler.
    // let arr = [3; 5]; // this is another way of initializing where if u want to repeat a
    // // specific number with n times, in this case number 3 for 5times.
    //
    // println!("{}", arr[0]);
    //
    // let str = String::from("Manikandan");
    // {
    //     let str2 = str.clone();
    //     println!("{str}");
    //     println!("{str2}");
    // }
    // println!("{str}");
    // let s = string::from("hello");  // s comes into scope
    // let s = takes_ownership(s);             // s's value moves into the function...
    // println!("{s}")
    let mut str = String::from("Manikandan");

    println!("{}", str); // Manikandan

    modify_str(&mut str); 

    println!("{}", str); // Manikandan Arjunan
}

fn modify_str(str: &mut String) {
    if str == "Manikandan" {
        str.push_str(" Arjunan")
    }
}
