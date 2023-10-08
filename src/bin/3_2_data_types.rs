// Data Types
// //////////
//
// Every value in Rust is of a certain data type.
// It's data type provides specific possiblities for how that data
// may be used.
//
// There are two species of data types: scalar and compound.
//
// Though Rust is a statically typed language, it can often infer
// the data type of a variable at compile time, given the value
// to which it is bound and the uses to which it is put.
//
// Scalar Types
// ////////////
//
// A scalar type represents a single value.
// Rust has four primary scalar types:
//      * integers
//      * floating-point numbers
//      * booleans
//      * characters
// 
// Integer Types
// /////////////
//
// An integer type is a number without a fractional component.
// Integer types may be signed or unsigned, and of 8, 16, 32, 64, or 128
// bits in size, types as i8 or u32 for signed 8-bit integer and unsigned
// 32-bit integer respectively.
//
// The `isize` and `usize` types depend on the architecture of the
// computer running the program - typically either 32 or 64 bit.
//
// Integer literals can be written in any of the following notations
//      * Decimal:  98_777
//      * Hex:      0xff
//      * Octal:    0o77
//      * Binary:   0b111-0000
//      * Byte:     b'A'
// Note that underscores may be used to separate grouping of numbers,
// functioning like commas in standard English decimal notation.
//
// Rust defaults to i32.
//
// `isize` and `usize` is typically used when indexing a collection
//
// Integer Overflow
// ////////////////
//
// Integer overflow is a result of an integer value larger than is available
// to the variable type (e.g. 257 for an u8).
// In the event of an overflow, the compiler will panic in debug mode, and
// will wrap around in production mode.
// Overflow behaviour can be specified with a family of methods provided
// for by the standard library.
//      * wrapping_*
//      * checked_*
//      * overflowing_*
//      * saturating_*
// 
// Floating-Point Types
// ////////////////////
//
// Rust has two primitive types for floating point numbers: f32 and f64.
// The default is f64 given modern CPUs typical 64-bit architecture.
//
// Boolean Types
// /////////////
//
// Rust has two possible boolean type values: `true` and `false`
// These are typically used in if expressions.
//
// Character Types
// ///////////////
//
// Rust has a primitive character type represented as `char`.
// Character literals are specified with the single-quotation
// mark, as opposed to the string literals which are specified
// with the double-quotation marks
//
// Rust character are four bytes in size, and thus represent much
// more than English character
//
// Compound Types
// //////////////
//
// Compound types can group multiple values into one type.
// Rust has two primitive compound types: tuples and arrays
//
// Tuple Type
// //////////
//
// Tuples group values of a variety of types into one
// compound type.
//
// Tuples have fixed length, fixed at the time of their declaration
//
// The tuple without any value, `()`, is the 'unit'.
// This value and its corresponding type are both written `()` and 
// represent an empty value or an empty return type.
// Expressions implicitly return the unit value if they don't return
// any other value.
//
// Arrays
// //////
//
// Arrays are a second compound type, but with elements of all the same
// type.
//
// Arrays are of a fixed length, unlike a vector, and so are well suited
// to use for such fixed length compound types
//
// Arrays types may be explicited during instantiation
//
// Accessing array elements is achieved similarly to tuples, but
// with square brackets
//
// Attempting to access an array whose index is out of range will cause
// the program, which can compile, to panic.
// This is an example of the safety of programming in Rust.
// It protects against random memory access.

use std::io;

fn main() {

    // Inferring data types
    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!");    // Absent the type
                                                            // annotation, the 
                                                            // compiler will
                                                            // throw an error
                                                            // given the 
                                                            // ambiguity of the 
                                                            // intended type of
                                                            // guess
    println!("The value of guess is: {guess}");

    // Underscores and digit groupers
    let x = 98_222;
    println!("The value of x is: {x}");

    // Scalars variables

    // Floating point scalars
    let x = 2.0;            // f64
    let y: f32 = 3.0;       // f32
    println!("The values of x and y are: {x} and {y}");

    // Boolean scalars
    let t = true;           // implicit type
    let f: bool = false;    // explicit type annotation
    println!("The values of t and f are: {t} and {f}");


    // Character scalars
    let c = 'z';
    let z: char = 'Z';      // with explicit type annotation
    // let heart_eyed_cat = ''; this would be an emoji if I could type them
    // in vim
    println!("The values of c and z are: {c} and {z}");

    // Compoung Types


    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Deconstructing tuples
    let (x, y, z) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("The value of tup[x], tup[y], tup[z] are: {x}, {y}, {z}");
    println!("The value of tup elements by dot notation are: {a}, {b}, {c}");

    // Arrays
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Accessing array elements
    let month_one = months[0];
    println!("The first month is: {month_one}");


    // Explicited array declarations
    let a: [i32; 5] = [1, 2, 3, 4, 5];


    let first = a[0];
    let second = a[1];

    println!("The first two values of array a are: {first}, {second}");

    // Accessing out of range index
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");
    
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
