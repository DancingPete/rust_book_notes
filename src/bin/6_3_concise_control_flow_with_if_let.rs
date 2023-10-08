#![allow(unused_variables,dead_code)]

// Concise Control Flow with `if` `let`
// ////////////////////////////////////
//
// The `if let` syntax provides for the combination of `if` and `let` in a
// less verbose manner than is provided for by the standard syntax in 
// handling an attempt to handle one `match` pattern while ignoring the rest.
//
// It is possible to include an `else` clause in an `if let` statement.


fn main() {

    // Here the second default arm is included to meet the exhaustive 
    // requirement of the `match` expression.
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configered to be {}", max),
        _ => (),
    }

    // Alternative `if let` syntax
    //
    // This is identical in function, but more succinct
    let config_max: Option<u8> = Some(3u8);
    if let Some(max) = config_max { println!("The maximum is configurered to be {}", max); }
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