// Variables and Mutability
// ////////////////////////
// 
// Variables
// /////////
// Variables in Rust are immutable by default.
// Variables may, however, be declared mutable.
//
// Once an immutable variable is bound to a value, its
// value cannot change.
//
// The compiler will produce an error where an immutable variable
// is mutated.
//
// Variables may be mutated with the `mut` keyword
//
// Constants
// /////////
//
// Constants are values that are bound to a name and are not allowed
// to change - like immutable variables.
// However, `mut` may not be used with constants, you declare a constant
// with the `const` keyword rather than the `let` keyword, and constants
// must be annotated upon declaration.
// Additionally, constants may only be set by a constant expression, not
// the result of a value that could only be computed at runtime
//
// The naming convention for constants is all uppercase with underscores 
// between words.
//
// Shadowing
// /////////
//
// It is possible to declare new variables with the same name as previously
// declared variables - this is called 'shadowing'.
// In doing so, the second declared variable of the same name 'overshadows'
// the first for the duration of its scope.
//
// Shadowing is distinct from mutability insofar as the compiler will throw
// an error if an immutable variable is accidentally reassigned without the
// use of the `let` keyword.
//
// Additionally, shadowing is effectively creating a new variable with the
// same name, meaining that the old and new variable may have different 
// types - unlike mutability which may change the value of a variable, but
// cannot change its type.

fn main() {
    
    // Using default immutable variables
    let x = 5;          // Variable set as immutable by default
    println!("The value of x is: {x}");
    // x = 6;           // Attempt to mutate immutable value will yield a
                        // compiler error
    println!("The value of x is: {x}");


    // Using mutable variables
    let mut x = 5;      // Variable set as mutable with the `mut` keyword
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // Defining constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing values allow you to reinitialize an existing, immutable
    // variable
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");

    // Shadowing allows variables to change type
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Conversly, repeating the exercise with a mutable variable
    // the variable is seen to be resistant to type change and will error
    // out at compile time
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // println!("The value of spaces is: {spaces}");
}
