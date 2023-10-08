// References and Borrowing
// ////////////////////////
//
// As an alternative to a function return both the original argument
// value and the functional result  of that function's operation on 
// the parameter as a tuple, it is possible to pass a reference
// to the value of that argument's value to the function instead of the
// argument itself (thereby avoiding any transfer of ownership).
//
// A reference is like a point in that it's an address we can follow
// to access the data dtored at that address; that data, however,
// is owned by another variable.
//
// Unlike a point, a reference is guaranteed to point to a valid
// value of a particular type for the life of that reference.
//
// If the only means of using head memory was via binding to 
// variables that could only, own or move ownership, the
// Rust API would be inconvient because, for instance, we
// often want to read variables more than once.
//
// References are the tool allowing us to overcome this limitation.
// References are non-owning pointers.
//
// When functions have reference type parameters, it is not necessary
// to return the argument in order to have that argument persist
// in the calling function.
// The creation of a reference is referred to as 'borrowing'.
//
// Trying to modify a borrowed value will yield a compilation
// error, because, just as for variables, references are immutable
// by default.
//
// Mutable References
// //////////////////
//
// It is possible to allow a function to mutate the value it receives
// as an argument by using a 'mutable reference' rather than a reference.
//
// Mutate reference have one significant restriction: you can have only
// one reference to that value.
// Code that attemps to create two mutable references will fail.
//
// However, if those mutable references are in different scopes, such
// that the one and the other never share the same scope and are released
// before they interact, no such compiler failure will follow.
//
// Similarly, we cannot have a mutable reference within the same
// scope as an immutable reference - though two immutable references
// are perfectly acceptable.
// This is sensible, as the users of an immutable reference cannot
// provide for a value that changes without knowledge.
//
// It should be noted that a reference's scope starts where it is
// introduced, and ends at its last use.
//
// Dangling References
// ///////////////////
//
// Typically, in most programming languages, it is easy to create 
// a dangling pointer - a pointer
// that references a location in memory that may have been given
// to something unexpected.
// In Rust, however, the compiler guarantees that references will
// never be left dangling.
//
// If you have a reference to some data, the compiler guarantees
// that the data will not go out of scope before the reference
// to the data does.
//
// Rules of Reference
// //////////////////
//
// 1. At any given time, it is possible to have either one mutable
//    reference or any number of immutable references.
// 2. References must always be valid.


fn main() {

    // Reference example
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // Notice that &s1 is passed as an argument, which parameter takes
    // by definition &String rather than a String.
    // These ampersands represent references, and they provide for
    // the reference to some value without appropriating ownership.
    println!("The length of '{}' is {}.", s1, len);

    // Mutable reference example
    let mut s = String::from("hello");

    change(&mut s);

    println!("Mutated String: {}", s);

    // Attempt to generate two reference to a mutable variable will
    // fail
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("r1: {}, r2: {}", r1, r2);

    // However, attempt to generate two reference to a mutable
    // variable will from different scopes will succeed
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
        println!("r1: {}", r1);
    }

    let r2 = &mut s;
    println!("r2: {}", r2);

    // Mutable and immutable reference in the same scope cause a
    // compiler failure
    // let mut s = String("hello");
    //
    // let r1 = &s;         // no problem
    // let r2 = &s;         // no problem
    // let r3 = &mut s;     // problem
    // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    //
    // If those mutable and immutable references do not share
    // a scope however, there is no problem
    //
    let r1 = &s;         // no problem
    let r2 = &s;         // no problem
    println!("r1: {}, r2: {}", r1, r2);

    let r3 = &mut s;     // problem
    println!("r3: {}", r3);

    // Attempt to create a dangling reference fails
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope, but because it does not own
  // the value to which it refers, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {             // dangle returns a reference to a String
//
//     let s = String::from("hello");   // s is a new string
//     &s                               // reference to s is returned
// }                                    // s goes out of scope and is 
                                        // and is dropped.
                                        // It's memory goes away.
                                        // The calling function 'reference'
                                        // would be to a dangling area of memory
