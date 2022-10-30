use std::io; // io input/output library
use std::cmp::Ordering; // Variants: Less, Greater, Equal // cmp method
use rand::Rng; // random number generator from random library
// $ cargo doc --open // to generate build docs of dependencies

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // start..=end inclusive on bounds

    //println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");

        // create a new empty instance of String type
        let mut guess = String::new();

        // get standard input from terminal
        // if not for line 1 can do std::io::stdin() instead
            // puts user input into the passed string // must be mut // & is reference
            // also returns a Result value // variants are Ok (success) and Err (operation failed)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // u32 = unsigned 32-bit int
         // trim \n or \r\n
                                //parse only works on characters no emoji's
                                // converts string to another type (u32)
        let guess: u32 = match guess.trim().parse() { // match retruns a variant 
                                    Ok(num) => num, // Ok(num), if success and convert to num, then return num
                                    Err(_) => continue, // Err(_), _ is a catch all value, continues to next loop
                                    // ignores all errors .parse() encounters and reruns the loop when it does
                                };
                                
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // returns a variant and arms will interpret the variant
            Ordering::Less => println!("Too small"), //arms
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// let apples = 5; // immutable
// let mut bananas = 5; // mutable


// String::new() 

/* Output formatting
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
*/