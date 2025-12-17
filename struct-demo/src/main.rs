struct Employee {
    name: String,
    age: i16,
}

fn main() {
    let emp1 = Employee {
        name: String::from("Manikandan Arjunan"),
        age: 27
    };
    println!("{}", emp1.name);
    println!("{}", emp1.age);

    let emp2 = Employee {
        age: 28,
        ..emp1
    };

    println!("{}", emp1.name); // this won't work
    println!("{}", emp1.age);
}
