fn main() {
    // Variable Scope
    let _s = "hello"; // this is a string literal where the value of the
        // string is hardcoded into the text of our program
    {
        //println!("{_s}");  // _s is not valid here, it's not declared yet
        let _s = "hello";    // _s is valid from this point forward

        // do stuff with _s
    }                       // this scope is now over, and _s is no longer valid

    // The String Type
    let mut s = String::from("hello");
    
    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s); // This will print 'hello, world!'

        // Strings can be mutated but literals cannot
            // They deal with memory differently

    // Memory and Allocation
    /*
        String Literals vs String type
        string literal:
            - know contents at compile time
            - text is hardcoded into the final executable
            - fast and efficient
                - but is immutable
        String type:
            - unknown contents and size at compile time
                - memory must be requested from memory allocator at runtime
                - must return memory to allocator when done with the String

        allocate:
            String::from, requests memory it needs
        free:
            use a scope {} so when variable becomes invalid outside the scope
    */
    {
        let _s = String::from("hello"); // s is valid from this point forward
        
        // do stuff with s
    }       // this scope is now over, and s is no longer valid
    // at the end of the scope Rust calls the drop function automatically

    // Ways Variabls and Data Interact: Move
        // bind the value 5 to _x
            // then make a copy of the value in x and bind it to y
    {
    let _x = 5;
    let _y = _x; // integers are simple values with a known fixed size
    // and these two 5 values are pushed onto the stack

    let _s1 = String::from("hello");
    let _s2 = _s1; 
    // performs a shallow copy of the pointer, length, and capacity
        // without copying the data; this is known as a move
            // then invalidates _s1 to prevent a double free error
                // we say: s1 was moved into s2

    //println!("{}, world!", s1) // get an error because s1 is invalidated
    }

    // Ways Variable and Data Interact: Clone
        // when we want to deeply copy the heap of data of the String not just the stack
    {
    let _s1 = String::from("hello");
    let _s2 = _s1.clone();

    println!("_s1 = {}, _s2 = {}", _s1, _s2);
    }

    // Stack-Only Data: Copy
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
        // since types like integers are stored entirely on the stack
            // we don't have to worry abour the cost of cloning the data
                // so creating deep copies are quick and done by default
                    // so x doesn't need to become invalid after binding its value to y
                        // aka don't have to call clone and x doesn't get moved to y
        
        // int (u32), bool, floats (f64), char, tuple if contains copy type
            // these all implement copy when assign a variable to another variable
    }

    // Ownership and Functions

    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);         // s's value moves into the function
                                        // and is no longer valid here

        // println!("{s}"); get an error since s has been dropped by the function

        let x = 5;                      // x comes into scope here

        makes_copy(x);                  // x would move into the function
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward
        println!("x = {x}"); // since x is still valid
    }   // Here, x goes out of scope, then s. But because s's 
        // value was moved, nothing special happens  
        
    // Return Values and Scope
    {
        let _s1 = gives_ownership();     // gives_ownership moves its return
                                        // value into s1

        let _s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(_s2);   // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value in to s3
    }   // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
        // happens. s1 goes out of scope and is dropped.

    {   // Let's try using passing a value to a function with out taking ownership
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and 'drop' is called. The backing
    // memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {                
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}   // too much ceremony and a lot of work
// references can use a value without transferring ownership.