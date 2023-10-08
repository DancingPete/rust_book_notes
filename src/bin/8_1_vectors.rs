fn main() {

    // Declaring a new, empty vector
    // Declaring without including any initial data required a type annotation
    let mut v1: Vec<i32> = Vec::new();
    println!("{}: {:?}", stringify!(v1), v1);
    
    // Declaring a new vector using the vec! macro
    // The type annotation is unnecessary because it is inferred from the macro argument
    let v2 = vec![1, 2, 3];
    println!("{}: {:?}", stringify!(v2), v2);

    // Vectors may be added using the push() method
    v1.push(5);
    v1.push(6);
    v1.push(7);
    println!("{}: {:?}", stringify!(v1), v1);

    // Value if a vector may be referred by index or using the get() method
    // depending on the manner according to which an attempt to reference an 
    // element past the end of the vector
    let third: &i32 = &v1[2];
    println!("The third element of {} is {}.", stringify!(v1), third);

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("Third third element of {} is {}.", stringify!(v1), third),
        None => println!("There is not third element.")
    }

    // Iterating over the values in a Vector
    for i in &v1 { println!("{i}"); }

    // Iterating over a mutable references to change the value of a vector's elements
    // To change the value of the elements, we must dereference the mutable 
    // reference to that element.
    for i in &mut v1 {
        *i += 50;
        println!("{i}");
    }

    // Using an enum to store multiple types
    // Because Rust needs to know the types being held in Vec, the enum allows
    // the program to determine a type which provides for variable sized elements
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
    println!("{} {:#?}", stringify!(row), row);

}