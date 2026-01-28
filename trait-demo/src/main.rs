struct Employee {
    id: i16,
    name: String,
}

struct Student {
    id: i16,
    name: String,
}

// Print is the trait(kinda interface),
// where Employee and Student are
// implementing the trait
trait Print {
    fn print_name(&self) -> String;
    fn print_id(&self) -> i16;
}

impl Print for Employee {
    fn print_name(&self) -> String {
        self.name.clone()
    }

    fn print_id(&self) -> i16 {
        return self.id;
    }
}

impl Print for Student {
    fn print_name(&self) -> String {
        self.name.clone()
    }

    fn print_id(&self) -> i16 {
        return self.id;
    }
}

// this print_details takes Print
// trait as input, and calls print_id and
// print_name, u can pass any struct that
// implemented print
fn print_details(role: &impl Print) {
    role.print_id();
    role.print_name();
}

// the above can also implmented using generics
fn print_details_2<T>(role: T)
where
    T: Print,
{
    role.print_id();
    role.print_name();
}

fn main() {
    let employee = Employee {
        name: String::from("Manikandan"),
        id: 1,
    };
    let student = Student {
        name: String::from("Manikandan"),
        id: 1,
    };

    print_details(&employee);
    print_details(&student);

    // this is also possible, u can also transfer the ownership of the struct
    print_details_2(employee);
    print_details_2(student);
}
