use std::{cell::RefCell, rc::Rc};

// use std::ops::Deref;
//
// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         // this is just mock, am not really storing this value in heap
//         MyBox(x)
//     }
// }
//
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }
//
// fn main() {
//     let box1 = MyBox::new(1);
//     let box2 = MyBox::new(String::from("Hello"));// this will first &MyBox(String) -> &String ->
//                                                  // &str, coz String and Mybox has deref
//                                                  // implemented
//
//     println!("{}", *box1);
//     println!("{}", *box2);
// }
//
//
// struct Student {
//     name: RefCell<String>,
// }
//
// trait Update {
//     fn update_name(&self);
// }
//
// impl Update for Student {
//     fn update_name(&self) {
//         self.name.borrow_mut().push('0');
//     }
// }
//
// fn main() {
//     let stu1 = Student {
//         name: RefCell::new(String::from("XXX"))
//     };
//
//     println!("Before Update -> {:?}", &stu1.name);
//     stu1.update_name();
//     println!("After Update -> {:?}", &stu1.name);
// }
//
//
//
//
//
// Rc -> shared state where it allocates the value on the heap one time
// and assigns the pointer to that heap for all the variables
// eg: let a = Rc::new(1 | String::from("Masda") || vec![asdasd]);
// all goes to heap, only change is a -> points to that heap.
// let b = Rc::clone(&a). doesn't clones the value of the heap
// instead clones the pointer value of a and store it to b.
// a -> heap
// b -> heap
// why &a -> rust borrow checker rules, if its Rc::clone(a), now a ownership got transferred into
// clone function, so a cannot be used further.
//
//
// RefCell -> if we want to mutate an immutable reference, rust provides a simple way with RefCell
// to mutate an immutable reference on run time.
//
// Arc -> shared state, but for threads.
fn main() {
    let a = Rc::new(RefCell::new(String::from("Manikandan")));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("Before Update");
    println!("{:?}", a);
    println!("{:?}", c);
    println!("{:?}", b);

    String::push_str(&mut a.borrow_mut(), " Arjunan");
    println!("After Update");
    println!("{:?}", a);
    println!("{:?}", c);
    println!("{:?}", b);
}












