fn main() {
    // PARAMETERS
    another_function(5, 'h');


    // STATEMENTS AND EXPRESSIONS
        //let x = (let y = 6); // causes error since let y = 6 does not return a value

    let _y = {
        let x = 3 ;
        x + 1 // expression have no semi colon and return a value
                // statements have a semi colon and do not return a value
    }; //expression evaluates to 4

    println!("The value of _y is: {_y}");

    // FUNCTIONS WITH RETURN VALUES
    let _x = five();

    println!("The value of _x is: {_x}");

    let _z = plus_one(5);

    println!("The values of _z is: {_z}")
}
// snake_case is concventional style
    // rust doesn't care where functions are defined (after is okay)
fn another_function(value : i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// must declare the return value of the function
fn five() -> i32 {
    5
}

fn plus_one(x : i32) -> i32 {
    x + 1
} 