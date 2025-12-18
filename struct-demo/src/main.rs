// struct Employee {
//     name: String,
//     age: i16,
// }
//
// fn main() {
//     let emp1 = Employee {
//         name: String::from("Manikandan Arjunan"),
//         age: 27
//     };
//     println!("{}", emp1.name);
//     println!("{}", emp1.age);
//
//     let emp2 = Employee {
//         age: 28,
//         ..emp1
//     };
//
//     println!("{}", emp1.name); // this won't work
//     println!("{}", emp1.age);
// }
//

// #[derive(Debug)]
// struct Rectangle {
//     width: u16,
//     height: u16
// }
//
// fn area_rect(area: Rectangle) -> u16 {
//     return area.width * area.height
// }
//
// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     dbg!(&rect);
//     println!("{}", rect.width);
// }
//

#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16
}

impl Rectangle {
    fn area(&self) -> u16 {
        return self.width * self.height;
    }
}

fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 20,
    };
    let area =  Rectangle::area(&r1);
    println!("{}",area);
    //
    // println!("Area: {}", r1.area());
    // println!("After value width: {}", r1.width); // this only works coz we have 
    // // borrowed rectangle as &self, normal self also works, but it transfers the ownership
}
