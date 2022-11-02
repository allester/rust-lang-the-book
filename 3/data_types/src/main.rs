// 3.2 Data Types

fn main() {
    // Rust is a statically typed langauge
    // must know the types of all variables at compile time
        // if we run the line below it will get an error saying to specify the type
    //let guess = "42".parse().expect("Not a number!");

    let _guess: u32 = "42".parse().expect("Not a number!");
    // added _ to guess because it is an unused variable

    // FOUR SCALAR TYPES: INTEGERS, FLOATING-POINT, BOOLEAN, CHARACTER

    // INTEGERS
        //defaults to i32
        /*
        signed: i8, i16, i32, i64, i128, isize | [-(2^bit-1), +(2^bit-1)]
        unsigned: u8, u16, u32, u64, u128, usize | [0, 2^bit]
            isize and usize depend on architecture 64 bit or 32 bit

        integer literals:
            decimal: 98_222 same as 98222 (can use _ as a visual seperator)
            hex: 0xff
            octal 0x77
            binary: 0b1111_0000
            byte (u8 only): b'A'

        integer overflow:
            u8: [0, 255] if assigning 256 there will be a panic! error
            compiling in --release will cause wrapping so 256 becomes 0 and 257 becomes 1
        */

    // Floating-Point Types
        // defaults to f64
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3; // f

    // multiplication 
    let _product = 4 * 30; // i

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0 | 2 and 3 are int so integer division occurs

    // remainder
    let _remainder = 43 % 5;

    // BOOLEAN
        // true or false
    let _t = true;

    let _f: bool = false; // with explicit type annotation 

    // CHAR
        // char tye is four bytes in size and represent a Unicode Scalar Value
    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // COMPOUND TYPES: TUPLE

    // TUPLE
        // fixed length and can have different element types
            // a tuple is considerd a single compound element
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
        // use pattern matching to destructure a tuple value

    let (_x, _y, _z) = _tup;
    println!("The value of y is: {_y}");

    // can access tuple elemnts by using a period follow by the index
    let _five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;

        // an empty tuple is called a unit and is written as () it returns empty

    // ARRAY
        // fixed length and every element must have the same type
    let _a = [1, 2, 3, 4, 5];
        // use for stacks || less flexible than vector since vectors can shrink or grow
            // use for when number of elements will not need to change

    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

}