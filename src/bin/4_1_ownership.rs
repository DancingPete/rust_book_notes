// Ownership
// /////////
//
// Ownership is a set of rules that govern the management of memory.
// Unlike like other programming languages that use the approach of
// 'garbage collection' or defer the responsibility to the programmer,
// Rust uses a system of ownership implemented and enforced by the
// compiler.
// Programs that breach the rules of ownership, will not compile.
// The compiler thereby ensures memory safety.
//
// Ownership Rules
// ///////////////
//
// There are three ownership rules:
//      * Each value in Rust has an owner
//      * There can only be one owner at a time
//      * When an owner goes out of scope, the value will be dropped
//
// Variable Scope
// //////////////
//
// A scope is the range within a program for which an item is valid.
// A variable is valid from teh point at which it is declare
// until the end of the current scope.
//
// The String Type
// ///////////////
//
// The String type is a useful case study for memory stored on the heap.
// Strings, as described here, will focus on their features that pertain
// to their aspects that related to ownership.
//
// Unlike a string literal, the `String` type manages data allocated on
// the heap, and thereby store an amount of text only know at runtime.
//
// String literals as fast and efficient because they are hardcoded, and
// their size may be determined at comipile time.
// However, where a string size is unknown until runtime, this can't be
// hardcoded into a binary executable.
// Instead, the memory must be requested from the heap at runtime, and
// must be returned to the allocator when it is no longer needed.
//
// The first step of requesting an allocation is done by the programmer
// by calling `String::from`.
//
// The deallocation, however, is different in Rust than it is in other
// programming languages.
// In languages with a garbage collector (GC), the GC keeps track of
// and cleans up memory that isn't being used anymore.
// In languages without a GC, it is the programmer's responsibility
// to identify when memory is no longer being used, and to call
// code to explicitly free it.
// Programmers are notoriously bad at doing this well.
// If the programmer forgets, they waste memory. 
// If the deallocated to early, they introduce an invalid variable
// If the deallocation is performed multiple times, this also
// introduces a bug.
// Exactly one call to `allocate` must be paired with one call to
// `free`.
//
// Alternatively, Rust returns memory automatically once the variable
// that 'owns' it goes out of scope.
//
// When heap memory allocation goes out of scope, Rust automatically
// calls the `drop` function.
//
// A String value allocated from the heap is comprised of three parts:
//      * a pointer to the memory that holds the contents of a string
//      * a length
//      * a capacity
//
// Variables and Data Interacting with Clone
// /////////////////////////////////////////
//
// The method `clone` allows for a deep copy of heap data and a new
// addressed stack pointer.
//
// Deep copies, however, are expensive.
// This is important when dealing with very large data.
//
// Stack-Only Data: Copy
// /////////////////////
// 
// Types sucn as integers, which have a know size at compile time
// are stored entirely on the stack, soc copies of the actual
// values are quick to make.
//
// Consequently, there's no difference between deep and shallow
// copies of such variables.
//
// Rust has a special annotation called the `Cope` trait that we
// can place on types that are stored on the stack.
// Wehre a type implements the `Copy` trait, variables that use
// it do not move, but rather are trivially coped, making them
// still valid after assignment to another variable.
//
// Rust won't allow annotating a type with `Copy` if the type implements
// the `Drop` trait.
// Where a type needs something special to happen when it goes out of 
// scope, using `Copy` on that type will result in a compile error.
//
// Any group of simple scalar values can implement `Copy`, and nothing
// requires allocation or is some form of resource can implement `Copy`.
//
// Ownership and Functions
// ///////////////////////
//
// The mechanics of passing a value to a function are similar to those
// when assigning a value to a variable.
// Passing a variable to a function will move or copy, just as assignment
// does.
//
// Return Values and Scope
// ///////////////////////
// 
// Returning values can also transfer ownership.
//
// The ownership of a variable follows the same pattern every time: assigning
// a value to another variable moves it.
// When a variable that includes data on the heap goes out of scope, the
// value will be cleaned up by `drop` unless ownership of the data has 
// been moved to another variable.
//
// While this work, taking ownership and then return ownership with every
// function is somewhat tedious.
//
// Rust does let us return multiple values using a tuple.


fn main() {
    
    // Variable scope
    // //////////////

    // s is a string literal, such that the value is hardcoded into the
    // program.
    // The variable is valid from the point at which it is declared until
    // the end of the current scope.
    let s1 = "hello";
    println!("s1: {}", s1);

    {                       // s2 is not valid here
        let s2 = "hello";   // s2 is valid from this point forward
        println!("s2: {}", s2);
        // do stuff with s2
    }                       // s2 is no longer valid, as the scope is over

    // A heap allocated String type variable may be created as follows:

    let s3 = String::from("hello");  // The double colon operator allow us
                                    // to namspace this particular `from`
                                    // function under the `String` type
    println!("s3: {}", s3);

    // A mutable heap allocated String type may be created as follows:

    let mut s4 = String::from("hello");
    s4.push_str(", world!");            // push_str() appends a literal to a 
                                        // String
    println!("s4: {}", s4);             // This will print `hello, world!`
    
    // Here a heap allocated String variable is seen to go out of scope
    // and be freed automatically

    {
        let s5 = String::from("hello");     // s is valid from this point
                                            // onward
        println!("s5: {}", s5);
        // do stuff with s5
    }                                       // s is now out of scope and so
                                            // becomes invalid
    
    let x = 5;      // bind the value of 5 to x
    let y = x;      // bind a copy of the value of x to y
                    // Now both x and y are bound to the value of 5
                    // and both are pushed onto the stack as known,
                    // fixed-sized values.
    println!("x: {}", x);
    println!("y: {}", y);

    let s6 = String::from("hello");
    let s7 = s6;                    // This pattern functions differently.
                                    // Here, the pointer address, length,
                                    // and capacity of the heap allocated
                                    // string are stored on the stack
                                    // as s6.
                                    //
                                    // When s6 is assigned to s7, the String
                                    // data is copied, which is to say
                                    // the pointer address, and capacity.
                                    // The heap allocated data containing the
                                    // actual string, however, is not copied.
                                    // Both String types hold the same pointer
                                    // address.
                                    //
                                    // To ensure that s6 and s7 do not both
                                    // try to free the heap memory as they
                                    // pass out of scope, which would intro
                                    // duce a bug as they both reference the
                                    // same memory address, Rust no longer
                                    // treats s6 as a valid reference.
                                    // Rust does not free anything when s6
                                    // goes out of scope
    

    // println!("{}, world!", s6);  // This will fail as s6 is no longer valid
    println!("{}, world!", s7);

    // Deep copy with clone
    let s8 = String::from("hello");
    let s9 = s8.clone();
    println!("s8: {}, s9: {}", s8, s9);

    // Basic type copy remaining on the stack
    let x = 5;
    let y = x;
    println!("x: {}, y:{}", x, y);


    let s = String::from("hello");      // s comes into scope
    
    takes_ownership(s);                 // s's values moves into the function...
                                        // ... and so is no onger valid here

    let x = 5;                          // x comes into scope

    makes_copy(x);                      // x would move into the function,
                                        // but i32 is Copy, so it's okay
                                        // to still use x after passing it
                                        // as an argument

    let s1 = gives_ownership();         //gives_ownership moves its return
                                        //value into s1

    println!("s1:{}", s1);
    
    let s2 = String::from("hello");     // s2 comes into scope
    
    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back
                                        // which also moves its return value
                                        // into s3

    println!("s3:{}", s3);
                                        
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

} // main                               // Here x goes out of scope, then s.
                                        // But because s's value was move,
                                        // nothing special happens

fn calculate_length(s: String) -> (String, usize) {

    let length = s.len();       // len() returns the length of a String
    (s, length)                                

}

fn gives_ownership() -> String {        // gives_ownership will move its return
                                        // value into the fuction that calls it

    let some_string = String::from("yours");    // some_string comes into scope

    some_string                         // some_string is returned and moves
                                        // out to the calling function
}


fn makes_copy(some_integer: i32) {              // some_integer comes into scope
    println!("makes_copy of some_integer: {}", some_integer);
}                                               // Here, some_integer goes out
                                                // of scope.
                                                // Nothing special happens.

fn takes_and_gives_back(a_string: String) -> String { 
    a_string        // a_string is returned and moves out to the calling
                    // function
}

fn takes_ownership(some_string: String) {       // some_string comes into scope
    println!("takes_ownership of some_string: {}", some_string);
}                                               // Here, some_string goes out of
                                                // scope and `drop` is called.
                                                // The backing memory is 
                                                // freed.
