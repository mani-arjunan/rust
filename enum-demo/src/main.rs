// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i16, y: i16 }, // enum also supports values to be provided
//     Write(String),
//     ChangeColor(i16, i16, i16),
// }
//
// impl Message {
//     fn call(&self){
//         println!("{:?}", self);
//     }
// }
//
// fn main() {
//     let m = Message::Write(String::from("Hello"));
//     println!("{:?}", m); //print Write("Hello")
// }
//

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => {
//                 let temp = i + 1;
//                 
//                 return Some(temp);
//             },
//         }
//     }
//
//     let five = Some(5);
//     let six = plus_one(five); // also not possible to pass 5, coz 5 is i16 not Option<i16>
//     // Option<i16> is either Some(i16) or None
//     let none = plus_one(None);
//
//     println!("{:?}", six);
//     println!("{:?}", five);
//     println!("{:?}", none);
// }
//

fn main() {
    let config = Some(3u8);
    // match config {
    //     Some(s) => println!("hello {}", s),
    //     _ => (),
    // }
    if let Some(s) = config {
        println!("{}", s);
    }
}
