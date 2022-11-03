use std::io;

//This Program will Generate the nth Fibonacci Number using recursion
fn main() {
    loop {
        println!("Please input a natural number.");

        let mut n = String::new();
        
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line.");

        let n : u32 =  match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let num : u32 = fib(n);

        println!("The {n}th Fibonacci number is {num}.");
    }
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n-1) + fib(n-2);
}