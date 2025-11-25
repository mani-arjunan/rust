// During the time when i wrote this, i have 0idea about rust, this program is just to
// understand some highlvel stuffs about rust also to get around with syntax and other stuff,
// recommended by the official rust book.
use rand::Rng;
use std::cmp::Ordering;
use std::io; // import standard io package

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // let mutable_var: i32 = 10;
    // mutable_var = 11; // cannot do this, coz by default all variables are immutable in rust
    //
    // println!("{mutable_var}");

    loop {
        let mut guess = String::new();
        println!("Type a number to guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // this can also be used by std::io::stdin()

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}, Please enter a number", guess);
                continue;
            }
        };
        // .expect("Please type a number");

        println!("Your guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is too low!"),
            Ordering::Greater => println!("Number is too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
