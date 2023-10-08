#![allow(unused_variables, dead_code)]

// Defining an Enum
// ////////////////
//
// While structs provide a way of grouping thematic data together,
// enums provide a way of organizing values as a set of options.
// For example, a rectangle may be one of a set of possible
// shapes, such as circles, rectangles, and triangles.
//
// enums are 'enumerations' of possible values.
// For example, an enum could enumerate the possible internet
// protocol addresses for a host as either of IPV4 or IPV6.
// Any address can be either one or the other, but not both.
//
// Enum Values
// ///////////
//
// Note that the variants of the enum are namespaced under its
// identifier, and we use a double colon to separate the two.
//
// The name of each enum variant that we define also becomes
// a function that constructs an instance of the enum.
// The variant name, thereby, becomes a callable constructor,
// where the parameters of the variant the 
//
// Interestingly, each variant of an enum can have a different
// type and quantity of data associated with it.
//
// The standard library implement a type of IpAddr, defining
// a struct for each protocol address, and an enum for constructing
// an IpAddr. 
// This example, see below, illustrates that enum variants may take any
// kind of data, including structs and enums.
//
// It is possible to implement methods on enums similarly to 
// the implementation of methods on structs.
//
// The Option Enum and Its Advantages Over Null Values
// ///////////////////////////////////////////////////
//
// The standard library define the `Option` enum.
// The `Option` type envodes the very common scenario
// in which a value could be something or it could be nothing.
//
// For example, if you request in the first item in a non-empty
// list, you would get a value; but, if you request the first
// item in an empte list, you would get nothing.
// Expressing this concept in terms of the type system means that
// the compiler can check whether you've handled all cases you 
// should be handling.
//
// Rust doesn't have the null feature that many other languages
// have.
// Null is a value that means that there is no value.
// In langauge that provide for null values, all values are 
// always in one of two states: null or not-null.
//
// The problem with null values is that if you try to use
// a null value as a not-null value, you'll get some kind
// of error - an easy mistake to make given the pervasiveness
// of the null/not-null property.
//
// Rather than a null type, Rust provides for the concept of the
// presence or absence of a variable.
// This implemented as an enum `Option<T>`.
//
//      enum Option<T> {
//          None,
//          Some(T),
//      }
// 
// The enum is so useful it is included in the prelude, and so
// doesn't need to be brought into scope explicitly.
// It's variants are also included in the prelude: `Some` and
// `None` may be used directly, without the `Option::` prefix.
//
// The advantage of the `Option<T>` is that the compiler wouldn't
// allow the compilation of code that attempt to add an `Option<T>`
// type that may contain either an `i32` value or no value with 
// an i32 value, because if it did, and during runtime the `Option<T>`
// type held no value, the program would crash.
// To approach the program, the program would have to check for null
// before proceding with the addition.
// The compiler, here again, guarantees that the code will run, 
// because it guarantees that any use of an `Option<T>` enum type
// will address the event of a null value.
// Specifically, the program would have to convert the `Option<T>`
// type to an `i32` type.

// This `enum` declaration creates a new custom data type.
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr1 {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Standdard library implementation of IpAddr
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

// Here is an example demonstrating that enum variants may be defined
// to accept any type of data type as its 
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Here is a enum with a variety of variants:
//  * Quit has no associate data
//  * Move has name fields, like a struct
//  * Write includes a single string
//  * Changecolor include three i32 values
// 
// This data type definition allows for the creation of a function
// that takes a 'Message' type, though that parameter in fact takes
// any of a number of different types grouped together.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Implementing methods on enums
impl Message{
    
    fn call(&self) {
        // method bod would be defined here
    }

}



fn main() {

    // Noting that the two variants are namespaced under its identifier
    // it is possible to create a function that takes any of the 
    // enum custom data type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Here, the enum is define to take parameters, functioning
    // as a constructor
    let home1 = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr1::V6(String::from("::1"));

    // Here, the variants of an enum may be of different types
    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    // Calling an enum method
    let m = Message::Write(String::from("hello"));
    m.call();

    // Example of using the Option enum
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

}
