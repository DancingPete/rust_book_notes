fn main() {
    let number = 3;

    // The condition of if statements must always result in
    // a boolean value.
    // Rust, unlike other languages, will not interpolate
    // boolean values from other value types.

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let another_number = 6;

    if another_number % 4 == 0 {
        println!("number is divisible by 4");
    } else if another_number % 3 == 0 {
        println!("number is divisible by 3");
    } else if another_number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Because if is an expression, it may be used within
    // a let statement

    let condition = true;
    let yet_another_number = if condition { 5 } else { 6 };

    println!("The value of yet another number is: {yet_another_number}");

    // Remember that blocks of code evaluate to the last expression they
    // contain.
    //
    // The values of the condition block must be of the same type, because
    // the compiler must know the type of each variable.
    // As the type is different given run conditions, it is not possible for
    // the compiler to determine the value of the if expression otherwise.
    // The following would result in an error:
    //
    // let condition = true;
    // let number = if condition {5} else {"six"};
    // println!("The value of number is: {number}");

    // Rust makes available three loop structures: loop, while, and for.
    //
    // loop will loop over a block until instructed not to.

    // Infinite loop
    // loop {
    //     println!("again!");
    // }

    // The 'break' keyword will break out of a loop
    // The 'continue' keyword will reinitialize the loop from the
    // start of the next iteration.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 { break counter * 2; }
    };

    println!("The final result is: {result}");


    // loops may be labelled to disambiguate the break statements
    // within them.

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8  { break; }
            if count == 2 { break 'counting_up; }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // The 'while' loop will run so long as the condition is true

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!!");

    // The 'for' loop provides for looping through a collection

    let a = [10, 20, 30, 40, 50];
    for element in a { println!("the value of a element is: {element}"); }
}
