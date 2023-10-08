fn main() {

    // Version 1 - Simple variables
    // Area function inteded to calculate area of one recture
    // whose parameters have not clear relation to each other
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectagle is {} square pixels.",
        area1(width1, height1)
    );

    // Version 2 - Tuple parameter
    // Improved by virtue of passing a single parameter representing
    // a single rectangle; however, this remains less than ideal as
    // tuple do not name their parameters.
    let rect1 = (30, 50);
    println!(
        "The area of the rectagle is {} square pixels.",
        area2(rect1)
    );

    // Version 3 - Using a struct
    // Here we have an improved version that takes a single parameter
    // rectangle, instance of a Rectangle type, borrowed in order
    // to perform the required calculation, and return the area.
    // This conveys that height and width are related to each other,
    // and it gives descriptive names to the values rather than usintg
    // the tuple index values 0 and 1.
    let rect2 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectagle is {} square pixels.",
        area3(&rect2)
    );

    // Testing print functionality
    // By default, curly brackets tell the println! macro to use formatting
    // known as `Dispaly:` output intended for direct, end user consumption.
    //
    // The primitive types seen so far all implement `Display`.
    //
    // With structs, however, no assumptions are made regarding how the
    // output is to be preferred.
    //
    // Similarly, the trait `Debug` is not implement by default for structs,
    // which would provide the options to print using the format specifier
    // `:?`.
    //
    // However, it is possible, as indicated in the compiler error message,
    // to add `#[derive(Debog)]` to the struct definition to provide for
    // default, debug formatting.
    println!("rect1: {:?}", rect2);

    // It is also possible to add the `:#?` format specifier to get
    // the default print of a struct broken over lines.
    println!("rect1: {:#?}", rect2);

    // Additionally, it is possible to use the `dbg!` macro, whcih takes
    // ownership of an expression (as opposed to `pintln!` which takes a
    // reference), which print to the stand output console stream `stdout`.
    dbg!(&rect2);

    // It's also possible to imbed the `dbg!` macro around an expression, which
    // because the macro return the value, has the expression return the 
    // value as well, as though the `dbg!` macro wasn't there.
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);
    for i in 1..5 { dbg!(i); }

    // In addition to the `Debug` trait, Rust has provided a number of
    // traits for us to use with the `derive` attribute that can add
    // useful behaviour to our custom types.
    //
    // Those traits and their behaviours are listed in Appendix C.
    //
    // Our area function is very specific - it only computer the area
    // of rectangles.
    // It would be helpful to tie this behaviour more closely to our `Rectangle`
    // struct because it won't work with any other type.
    // This is achieved by converting the function into a struct method.
}

#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

fn area1(width: u32, height: u32) -> u32 { width * height }
fn area2(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }
fn area3(rectangle: &Rectangle) -> u32 { rectangle.width * rectangle.height }
