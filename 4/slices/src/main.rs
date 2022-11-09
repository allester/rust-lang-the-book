fn main() {
    // The Slice Type
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5'

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize { //use &String since we don't want ownership
    // converts the String to an array of bytes
    let bytes = s.as_bytes();

    // iter() returns each element in a collection
    // enumerate() wraps the result of iter and returns
        // each element of as part of a tuple instead (index, reference to element)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // byte literal syntax for space
            return i;
        }
    }   // if a space is not fonud by end of loop it returns the len of string

    s.len()
}