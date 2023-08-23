// Functions
// /////////
//
// Rust uses snake case as the conventional style for function and
// variable names.
//
// A function is defined by the keyword `fn`, followed by a function
// name and a set of parentheses.
// The curly brackets define the beginning and end of the function
// body.
//
// A function is called by entering its name, followed by a set of
// parentheses.
//
// Rust functions may be defined in any order, and called from any
// location, so long as the definition is within scope
//
// Parameters
// //////////
//
// Functions may be defined to take 'parameters' - variables
// that are part of a function's signature.
// When a function has parameters, you can provide it with
// concrete values for those parameters.
// The concrete values are called 'arguments'.
//
// Multiple parameters are included in a function definition
// by separating each with a comma.
//
// Functions bodies are made up of a series of statements,
// optionally ending in an expression.
// So far, the functions defined haven't included an ending
// expression, though there have been expressions included
// within statements.
//
// Statements and Expressions
// //////////////////////////
//
// Because Rust is an expression-based language, the distinction
// between expressions and statements is important.
// 
// Statement ::= instructions that perform some action and do not
//               return a value
// Expression ::= evaluate to some resultant value
//
// Variable and function definitions are examples of statements.
// Statements do not return values.
// For this reason, it is not possible to assigned a 'let' statement
// to a variable such as let x = (let y = 6);
// Because 'let y = 6;' does not return a value, there is nothing
// to which the variable x in 'let x = (let y = 6);' may bind.
//
// Expressions may be part of a statement, and functions calls
// are an expression, just as calling a macro.
//
// Note that expressions do not terminate in a semicolon.
// Ending an expression with a semicolon reders it a statement,
// and thus no longer retuns a value.
//
// Expressions may be define as surrounded by curly braces, for
// instace:
//
// {
//      let x = 2;
//      x + 1
// }
//
// Functions can return values to the code that calls them.
// Return values aren't named, bu their types are declared with the 
// arrow '->'.
// In Rust, the return value of the function is synonymous with
// the value of the final expression in the block of the body of
// a function.


fn main() {
    println!("Hello, world!");

    // Calling functions
    another_function();
    yet_another_function(5);
    print_labelled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let z = plus_one(5);

    println!("The value of z is: {z}");
}

// Function definition
fn another_function() {
    println!("Another function");
}


// Adding a parameter to a function definition
fn yet_another_function(x: i32) {
    println!("The value of x is: {x}");
}

// Adding multiple parameters to a function definition
fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
