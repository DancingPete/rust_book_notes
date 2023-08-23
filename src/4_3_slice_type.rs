// Slice Type
// //////////
//
// Slices let you reference a continguous sequence of elements in a 
// collection rather than the whole collection.
// A slice is a kind of reference, so it does not have ownership.
//
// The Problem
// ///////////
//
// Slices are attempt to remedy the problem described below, of calculated
// variables that are functions of a data state without being tied to that
// state.
//
// The Solution - String Slices
// ////////////////////////////
//
// Rather than referencing an entire string, a string slice references
// only a portion of that string.
// Slices are created with a range within brackets, specifying the 
// `[starting_index..ending_index]`
//
// Where the starting index is 0, the `starting_index` may be dropped:
// `[..ending_index]`
//
// Where the ending index is `string.len()`, the `ending_index` may be
// dropped:
// `[starting_index..]`
//
// For the full string:  `[..]`
//
// String Literals as Slices
// /////////////////////////
//
// The type of s here, `let s = "Hello, World!";`, is `&str`.
// It is a slice pointing to that specific point of the binary.
// Defining functions parameters as `&str` types rather than `&String`
// types makes those function more broadly applicable as they make 
// take references to both strings and string slices as arguments.
//
// Other Slices
// ////////////
//
// There are other types of slices, such as array slices, which are 
// defined using the same index range notation.
//
// Summary
// ///////
//
// The concepts of ownership, borrowing, and slices ensure memory safety
// in Rust programs at compile time.
// The Rust language gives you control over your memory usage in the same
// way as other systems programming languages, but having the owner of data
// automatically clean up that data when the owner goes out of scope means
// you don't have to write and debug extra code to get this control.

fn main() {

    // Calculated variables untied to the state of the state from which they 
    // were calculated
    let mut s = String::from("hello world");
    
    let word = first_word(&s);      //word will get the value 5
    let word2 = first_word_2(&s);    //word will get the value 5
    let word3 = first_word_3(&s);    //word will get the value 5

    println!("s: {s}, word: {word}, word2: {word2}, word3: {word3}");
    
    s.clear();                      // this empties teh String, making it 
                                    // equal to ""
    
    // word still has the value 5 here, but there's no more string with which
    // we could meaninfgully use the value 5.
    // word is invalid, though there is no suggestion of this in the code
    // as implemented.
    //
    // word would still be valid if called, as its value is simply a usize 
    // and remains completely independent the string s.
    //
    // Planning for this incoherence is tedious and error prone.
    // This is truer if attempting to implement getting the second word in the
    // same fashion with a function such as: 
    //      fn second_word(s: &String) -> (usize, usize)
    // In this case, three unrelated variables are calculated from data
    // in a particular state without being tied to that state.

    // Alternative approach - String slice
    let s = String::from("hello world");
    let hello = &s[0..5];           // alternatively: let hello = &s[..5];
    let world = &s[6..11];          // alternatively: let world = &s[6..];
    // for the full string using string slice index notation
    let full_string = &s[..];

    println!("s: {s}, hello: {hello}, world: {world} full_string: {full_string}");

    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // println!("a: {}, slice: {}", a, slice);
}

// The function as an example of trying to identify the first word
// by identifying the first instance of a space in the string is problematic
// because the return value, an unsigned integer of architecture size, is
// only meainingful in relation to the string address supplied as an argument.
//
// Because it's a separate value from the string, there no guarantee that
// it will be valid in the future of the program.
fn first_word(s: &String) -> usize {

    let bytes = s.as_bytes();                       // Create an array of bytes 
                                                    // from a string using 
                                                    // as_bytes method

    for (i, &item) in bytes.iter().enumerate() {    // Iterate over each byte
                                                    // to test for a space, 
                                                    // including the index of
                                                    // each byte to return that
                                                    // index in case of match

        if item == b' ' { return i }                // Return index on find

    }

    s.len()                                         // Return length of array
                                                    // where no space is found
}

// Alternative implementation of first_word using string slice
// notation
fn first_word_2(s: &String) -> &str {
    
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }

    &s[..]
}

// Third alternative implementation of first_word using updated 
// signature for the parameters as &str rather than &String
// The change allows the function to be used with both &String
// and &str
fn first_word_3(s: &str) -> &str {
    
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }

    &s[..]
}
