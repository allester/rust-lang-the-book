fn main() {
    // References and Borrowing
        // take the last function of 4.1
            // the String must be returned by the fn so we can still use String
                // we can use references so we don;t have to do this
    {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //let s = String::from("hello");
    //change(&s); // doesn't work since we are borrowing s
    }

    // Mutable References
    {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");

    // cannot make two mutable references to a single value
    /*
    let r1 = &mut s;
    let r2 = &mut s; // error on second mutable borrow occurs here

    println!("{}, {}", r1, r2);
    */
        // the restriction prevents data races
    // use curly brackets for multiple references
    {
        let _r1 = &mut s;
    }   // r1 foes out of scope here, so we can make a new reference with no problems
    let _r2 = &mut s;
    }

    {   // can have multiple immutable references but cannot have a
            // mutable reference when there is an immutable reference
        let mut _s = String::from("hello");

        let _r1 = &_s; //no problem
        let _r2 = &_s; //no problem
        //let _r3 = &mut _s; // BIG PROBLEM

        //println!("{}, {}, and {}", _r1, _r2, _r3);
    }

        // a reference's scope starts from where it is introduced and continues
            // through the last time that reference is used
    {
        let mut s = String::from("hello");
        
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // varaibles r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    // Dangling References
    {   // dangling pointer: a pointer that references a location in memory that may
            // have been given to someone else
        // rust prevents compiling if there are dangling references

        //let reference_to_nothing = dangle();

            // dangle returns the pointer to s but s is dropped

            //solution is to return the value s instead of the reference &s
        let _s = no_dangle();
    }
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, s1 is not dropped
        // borrowing: action of creating a reference

/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*  Rust won't compile this function
fn dangle() -> &String {    //dangle returns a reference to a String
    let s = String::from("hello");  // s is a new String

    &s  // we retrun a reference to the String, s
}   // Here, s goes out of scope, and is dropped. Its mempry goes away/
    // Danger!
*/
fn no_dangle() -> String {  // solutiion is to retrun the String directly
    let s = String::from("hello");

    s
}

// The Rules of References
    // At any given time, you can either have one mutable reference or any number of
        // immutable references.
    // References must always be valid