#![allow(unused_variables,dead_code)]

// The `match` Control Flow Construct
// //////////////////////////////////
//
// Rust has an extremely powerful flow construct called
// `match` that allows you to compare a value against a series
// of patterns and then execute code based on which pattern matches.
//
// Pattern can be mae up of literal values, variables names, wildcards,
// and many other things.
// Chapter 18 covers all the different kinds of patterns and what they do.
// The power of `match` comes from the expressiveness of the patterns 
// and the fact that the compiler confirms that all possible cases
// handled.
//
// The `match` expression can be conceived as a coin-filtering machine:
// coins slide down a track with variously sized holdes along it, and each
// coing falls through the first hole it encounters into which it fits.
//
// Where an `if` condition must evaluate to a boolean expression, a `match`
// expression may evaluate to any type.
//
// The code with each arm is an expression, and the resultant value of the
// expression in the matching arm is the value that gets returns for the 
// entire match expression.
// Curly braces aren't typically included in the match expression; however,
// any attempt to include multiple lines of code must be so enclosed.
//
// Patterns That Bind to Values
// ////////////////////////////
//
// Match arms may bind to the parts of the values that match the pattern,
// permitting the extraction of values from enum variants.
//
// Matching with Option<T>
// ///////////////////////
//
// `match` provides a convenient implementation of `Option<T>` variants.
// Depending on whether or not the `Option<T>` is some or none, a function
// taking such a type as a paramter may use the `match` construct to
// handle either possiblity, and thereby ensure that the program compiles
// by handling all possiblities.
//
// Matches Are Exhaustive
// //////////////////////
//
// The arm's patterns of a `match` expression must be exhaustive.
// A match expression that does not consider all possible conditions
// won't compile, and the compiler will provide an error message that
// lists the unhandled possiblities.
//
// Catch-all Pattern and the _ Placeholder
// ///////////////////////////////////////
//
// The `match` expression provides for a 'default' arm using `other` as
// a branch name.
// The catch-all `other` arm ensures that the `match` expression requirement
// that the arms be exhastive is met.
//
// Rust also has a pattern that provides for avoiding any need to use the
// value in the catch all pattern: `_`
// `_` is a special pattern that matches any value and does not bind
// to that value.
// This pattern informs the compiler that the value won't be used, and
// so avoids the output of any warning regarding an unused variable.

fn main() {

    // Use of match expression to handle `Option<T>` function parameter
    // implementation
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?} six: {:?} none: {:?}", five, six, none);

    // Including a default within a match expression
    //
    // Here, a hypothetical roll of the dice is treated in one way
    // if the dice_roll is a 3, another if the dice_roll is a 7, 
    // and the same way irrespective of the dice_roll for all other
    // possible u8 values.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Including a default without binding the `match` test expression
    //
    // Here the `match` expression meets the exhaustive requirement, and
    // avoids any warning of an unused variable by relying on the `_`
    // pattern
    let dice_roll = 10;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => roll_again(),
    }

}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        // Match expression enclosed in curly braces given the use
        // of multiple lines of code.
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Here, when a Coin::Quarter matches, the `state` variable will
        // bind to the value of that quarter's state.
        // Then we can use state in the code for that arm.
        //
        // If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska)`,
        // `coin` would be `Coin::Quarter(UsState::Alaska)`.
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {

    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Sample functions to demonstrate use of match expression with
// default action
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn roll_again() {}
