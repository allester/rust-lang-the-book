use std::io;

fn main() {
    let mut f = String::new();
    loop {
        println!("Please input a number in Fahrenheit!");
        
        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line.");

        let f:f32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Your temperature is {f} degrees in Fahrenheit");

        let c : f32 = f_to_c(f); //pass by value not ref since we are not changing it
        println!("Your temperature is {c} degrees in Celcius");
        break
    }
}

fn f_to_c(f : f32) -> f32 {
    (f - 32.0) * (5.0/9.0)
} 
