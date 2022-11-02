fn main() {

    // if EXPRESSIONS

    let number = 3; //prints condition was true
    //let number = 7; // prints condition was false


    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // condition must be a bool unlike python
    /* causes an error because expected a bool not an integer
    if number {
        println!("nuber was three");
    }
    */

    if number != 0 {
        println!("number was something other than zero");
    }


    // HANDLING MULTIPLE CONDITIONS WITH else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    } // using too many else if clutters code so we can use match for these cases

    // USING if IN A let STATEMENT
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // if and else must have compatible types
    //let number = if condiiton { 5 } else { "six" }; // causes error since else evaluates to a str

    println!("The value of number is: {number}");

    // REPITION WITH LOOPS
    /* use control c on mac to end infinite loops
    loop {
        println!("again!");
    }
    */

    // RETURNING VALUES FROM LOOPS
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // LOOP LABELS TO DISAMBIGUATE BETWEEN MULTIPLE LOOPS
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    // CONDITIONAL LOOPS WITH while
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!");

    // LOOPING THROUGH A COLLECTION WITH FOR
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for LOOP USING A RANGE
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!");
}


