fn main() {

    // Strings are implemented as a collection of bytes, along with some
    // methods to provide convenient functionality.

    // Rust has only one strng type in the core language, which is string
    // slice `str`, typically handled in its borrowed form `&str`.

    // The `String` type, which is provided by the standard library is a 
    // growable, mutable, owned, UTF-8 encoded string type.

    // Creating new string similarly to an empty vector
    let s1 = String::new();
    println!("{} {}", stringify!(s1), s1);

    // Creating a string from initial data
    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();
    println!("{} {}", stringify!(s2), s2);
    println!("{} {}", stringify!(s3), s3);

    // Creating a string from a string literal
    let s4 = String::from("initial contents");
    println!("{} {}", stringify!(s4), s4);

    // Growing a string with the push_str() method
    let mut s6 = String::new();
    s6.push_str("Extending a string further");
    println!("{} {}", stringify!(s6), s6);

    // Growing a string with the push() method
    let mut s7: String = String::new();
    s7.push('A');
    println!("{} {}", stringify!(s7), s7);

    // Combining string with the '+' operator
    // Note that the first operand takes ownership and the second must pass only
    // a reference to the value
    let s8: String = String::from("Hello, ");
    let s9: String = String::from("World!");
    let s10: String = s8 + &s9;
    println!("{} {}", stringify!(s10), s10);

    // Indexing into strings
    // Rust strings don't support indexingA
    // A `String` is a wrapper over a `Vec<u8>` type.
    // String use UTF-8 encoding, and so the size of any character may not be
    // only 8 bits.

    // Methods for iterating over strings
    // Be explicit on whether you want to iterate over characters or bytes
    for c in "été".chars() {
        println!("{c}");
    }
    for b in "été".bytes() {
        println!("{b}");
    }

}