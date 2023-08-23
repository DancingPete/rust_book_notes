fn main() {
    let temp_c: f64 = 0.0;
    let temp_f: f64 = 0.0;
    println!("{temp_c} in Fahrenheit: {}", c_to_f(temp_c));
    println!("{temp_f} in Celsius: {}", f_to_c(temp_f));

    let n: u64 = 13;
    println!("The {n}th fibonacci number is: {}", fibo_n(n));

    twelve_days();
}

fn c_to_f(temp_c: f64) -> f64 {
    temp_c * 9.0 / 5.0 + 32.0
}

fn f_to_c(temp_f: f64) -> f64 {
    (temp_f - 32.0) * 5.0 / 9.0
}

fn fibo_n(n: u64) -> u64 {
    if n == 1 {
        0
    } else if n == 2 {
        fibo_n(n - 1);
        1
    } else {
        fibo_n(n - 2) + fibo_n(n - 1)
    }
}

fn twelve_days() {
    let gifts = [
        "A patridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..13 {
        println!("On the {day}th day of Christmas my true love gave to me:");
        for n in (0..day).rev() {
            let gift = gifts[n];
            println!("\t{gift}");
        }
    }
}
