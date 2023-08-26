// Method Syntax
// /////////////
//
// Methods are similar to functions: they are declared with the `fn`
// keyword and a name.
// They can have parameters and return a value, and they contain some
// code that's run when the method is called from somewhere else.
//
// Unlike a function, methods are defined with teh context of a struct
// (or an enum or trait), and the first parameter is always self.
//
// Defining Methods
// ////////////////
//
// To define a function within the context of a `struct`, we start an
// `impl` (implementation) block.
// Everything within this `impl` block be associated with the given
// `struct` type.
// The first parameter of any method is `self` - a reference to an
// instantiated `struct` type.
// In the signature of the method, a reference to self (&self).
// `&self` is actually short for `self: &Self`, where `Self` is an
// alias for the type that for which the `impl` block is an implementation.
//
// Having a method that takes ownership of the instance by using `self` 
// rather than `&self` is rate, and typically inteded to transform the
// instance into somthing new, preventing the caller from referencing
// the previous version.
//
// The primary purpose of using methods over functions is to make plain
// the functionality of a `struct`, rather than requiring the programmer/user
// to decipher the relationship between structs and functions.
//
// The method is called using dot notation.
//
// It is possible to name a method using the same name as one of its
// fields.
// Rust is able to distinguish between fields and methods given the
// presence of parentheses.
//
// Rust does not implement 'getters' automatically - methods of the
// same name as fields that only return the value.
// Getters are useful to make a field private, but the method public,
// and thus enable read-only access to that field.
//
// Methods with More Parameters
// ////////////////////////////
//
// Associated Functions
// ////////////////////
//
// All functions within an `impl` block are called 'associated functions'
// because they are associated with the type named after the `impl` keyword.
// It is possible to define function that don't have `self` as their first
// parameter.
// These are not methods because they don't require an instance to be
// called.
// One such function is the `String::from` function, a function associated
// to the String type.
//
// Associated functions are often used as constructors the will return a
// new instance of a `struct`.
// These are often called `new`, but `new` isn't a special keyword
// reserved by Rust.
//
// The `Self` keyword is an alias for the type that appears after the
// `impl` keyword.
//
// To call an associated function, use the `::` syntax with the struct
// name.A
//
// Multiple `impl` Blocks
// //////////////////////
//
// Each struct is permitted to have multiple `impl` blocks.
// There is no reason to separate methods and associated functions
// in separate `impl` blocks, but it is valid syntax.
//
// Summary
// ///////
//
// Structs let you create custom types that are meaningful for
// a domain.
// by using structs, assocaited pieces of data may be kept 
// associated to each other and named.
// In `impl` blocks, it is possible to define functions associated
// with the struct type, specifying the behaviours of those types.


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { 

    fn area(&self) -> u32 { self.width * self.height } 

    // Methods with the same name as a field
    fn width(&self) -> bool { self.width > 0 }

    // Methods taking additional parameters
    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height 
    }

    // Constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    let rect1 = Rectangle { width: 30, height: 50, };
    let rect2 = Rectangle { width: 10, height: 40, };
    let rect3 = Rectangle { width: 60, height: 45, };

    // Construct a new Rectangle using the square associated function
    // as a constructor
    let rect4 = Rectangle::square(10);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square height: {}, Square width: {}", rect4.height, rect4.width);


}
