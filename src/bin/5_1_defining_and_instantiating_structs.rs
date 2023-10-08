#![allow(unused_variables,dead_code)]

// Defining and Instantiating Structs
// //////////////////////////////////
//
// Structs are similar to tuples in that both hold multiple 
// related values, and both may be comprised of different types.
//
// Unlike tuples, structs name each piece of data, such that the order
// of the struct data is irrelevant.
//
// The definition of struct start with the keyword `struct`, followed
// by the name of the entire struct.
// Then, within curly braces, each data element is defined by a 
// name and type pair, which are together called the struct fields.
//
// To use a struct after it's been defined, we create an instance
// by specifying concrete values for each field.
// The instance is initialized by stating the name of the struct,
// then adding key value:pairs within curly braces, where the keys
// are the names of the fields set in the struct definition.
//
// To access instance data we use dot notation.
// If the instance is mutable, we can use dot notation to change
// the value of a particular field.
//
// It is import to note that the entire instance must be mutable
// for any of an instance's fields to be mutable.
//
// As with any expression, we can construct a new instance of the
// struct as the last expression in a function body to implicitly
// return that new instance.
//
// User the Field Init Shorthand
// /////////////////////////////
//
// Because it makes sense to name the function parameters with the 
// same name as the struct fields, and having to repeat the parameter
// and field names within the function body is tedious, Rust offers
// a convenient shorthand.
//
// Creating Instance from Other Instances with Struct Update Syntax
// ////////////////////////////////////////////////////////////////
//
// It is often useful to create a new instance of a struct that includes
// most of the values from another instance, with a few changes.
// Rust offers the struct update syntax for this purpose.
//
// Note that the struct update syntax uses `=`, like an assignment,
// and do so because it moves ownership of the data.
// After creating user3 using the struct update syntax, user1 is no
// longer available as a whole, because the ownership of the
// username field from user1 was moved to user2.
// If both the username and email fields were initialized for user3
// with new data, then the entirety of user1 would remain valid, because
// active and sign_in_count are both copy types.
//
// Using Tuple Structs without Name Fields to Create Different Types
// /////////////////////////////////////////////////////////////////
//
// Rust also support structs that look similar to tuples called 
// tuple structs.
// Tuple structs have the added meaning the struct name provides,
// but don't have named fields, rather than just have typed fields.
//
// Tuple structs are useful when you want to name a whole tuple to
// distinguish it from the generic tuple type, but where naming the
// fields would be redundant.
//
// Each struct defines a type, even though the fields within those
// struct definitions might be same set of types.
// For example, a function that takes a Point struct tuple would not
// accept a Color point tuple, even though each is comprised as the
// same set of unamed field types.
//
// Tuple structs may be accessed with dot notation, using an index
// value as the field name.
//
// Unit-Like Struct without Any Fields
// ///////////////////////////////////
//
// Structs without fields, named unit structs, behaving similar to `()` -
// the unit type.
// Unit like structs can be useful when you need to implement a trait
// on some type but don't have any data that you want to store
// in the type itself.

// Struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Defining a Unit-like struct
struct AlwaysEqual;

fn main() {

    // Struct instantiation
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Changing struct data for mutable instances
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    // Instantiating a struct from a function
    let pierre1 = build_user(
        String::from("pierre"),
        String::from("p.cassidy@fart.com")
    );
    let pierre2 = build_user_shorthand(
        String::from("pierre"),
        String::from("p.cassidy@fart.com")
    );

    // Struct update syntax
    // The `..instance_name` must come at the end of the field list
    // definition
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);    // Note that black and origin are different
                                    // types because they are instances of
                                    // different tuple structs, even though 
                                    // they would be identical tuple types
                                    // had they been defined as generic 
                                    // tuples.

    // Instantiating a unit-like struct
    let subject = AlwaysEqual;
}

// Return a new struct instance from a function
fn build_user(email: String, username: String) -> User {

    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Return a new struct instance from a function using shorthand
fn build_user_shorthand(email: String, username: String) -> User {

    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
