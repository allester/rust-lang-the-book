fn main() {
    //mutability
    //let x = 5; compile error since immutable
    let mut x = 5; // must have mut in order to change x from 5 to 6
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let y = 5;

    let y = y + 1; // shadowing: reinitialize y by using let on an immutable variable
    
    { // inner scope // can shadow variables in an inner scope (block {}) as well
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    // outside inner scope returns to original variables
    println!("The value of y is {y}");

    // shadowing // can change immutable variables by reusing the let variable
    let spaces = "   ";
    let spaces = spaces.len();

    // gets compile-time error since we're not allowed to mutate a variable's type
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
    */
}
