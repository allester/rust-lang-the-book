use std::io; // io input/output library

fn main() {
    println!("Hello, world!");

    println!("Please input your guess.");

    // create a new empty instance of String type
    let mut guess = String::new();

    // get standard input from terminal
    io::stdin() // if not for line 1 can do std::io::stdin() instead
        // store input into guess // & is reference
        .read_line(&mut guess); // pass mut guess as an argument to read_line # must be mut
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

// let apples = 5; // immutable
// let mut bananas = 5; // mutable

// String::new() 