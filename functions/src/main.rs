
// params as usual specify type, for return type use -> arrow operator
// fn add(x: i16, y: i16) -> i16 {
//     return x + y;
// }


// if u see below i have not included semi colon, this is intentional
// x + y is basically an expression so it returns(similar to () => 1 in js)
// if u include semi colon x + y; this becomes statement, so rust returns default
// value () empty tuple
fn add(x: i16, y: i16) -> i16 {
    x + y
}

// fn is the keyword, main is the default function that rust calls automatically on any file
fn main() {
    let num = add(1, 2);

    if num > 3 {
        println!("Greater than 3");
    } else {
        println!("Lesser than 3");
    }

    
    let final_number = if num > 3 { 3 } else { 5 };

    println!("{final_number}");
    println!("Hello, world!");

    // loop

    let arr = [1, 2, 3, 4, 5];

    for num in arr {
        println!("{num}");
    }

    for i in 0..arr.len() {
        println!("index => {i} value => {}", arr[i]);
    }
}
