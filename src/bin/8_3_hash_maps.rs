use std::collections::HashMap;

fn main() {

    // HashMap<K, V> stores a mapping of keys to values using a hashing function.
    // Hash maps are useful when accessing arrays using a name rather than an
    // index number.

    // Creating a new HashMap
    // Note that HashMap is not included in the prelude, nor is it
    // provided with a built-in, constructor macro
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{} {:?}", stringify!(scores), scores);

    // Accessing values in a HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{} {}", stringify!(score), score);

    // Iterating over the elements of a HashMap
    // The elements will print in an arbitrary order
    for (key, value) in &scores { println!("{key}: {value}"); }

    // HashMap and ownership
    // Elements not implementing the Copy trait will be passed to the 
    // hash map 
    let field_name: String = String::from("Favority colour");
    let field_value: String = String::from("Blue");
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}: {}", field_name, field_value); // This will yield an error as the ownership has
                                                    // transferred to map

    // Updating a HashMap
    // Each key must be unique and have only one value associated to it.

    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Adding a value only if the key is absent
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a value vased on an existing value
    let text = "Hello world wonderful world";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map2);

}
