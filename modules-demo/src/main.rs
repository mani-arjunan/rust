mod user;
mod employee;


use user::set_user_details;
use employee::it;
use employee::civil;

fn main() {
    let mut u = set_user_details("Manikandan", 1);
    println!("{:?}", u);

    user::update_user_details(&mut u, Some("Manikandan Arjunan"), Some(2));
    println!("{:?}", u);

    user::update_user_details(&mut u, None, Some(21));
    println!("{:?}", u);

    let mut mani_wife = civil::set_emp_details(String::from("mani_wife"), civil::CivilRole::MODELER, 1);
    println!("{:?}", mani_wife);

    civil::update_emp_details(&mut mani_wife, None, Some(civil::CivilRole::DETAILER), Some(3));
    println!("{:?}", mani_wife);

    let mut mani = it::set_emp_details(String::from("mani"), it::ItRole::SDE1, 1);
    println!("{:?}", mani);

    it::update_emp_details(&mut mani, None, Some(it::ItRole::SDE3), Some(3));
    println!("{:?}", mani);
}

