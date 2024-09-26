use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.insert(String::from("Blue"), 10);
    
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("{scores:?}");
    let mut map = HashMap::new();

    // Insert some key-value pairs into the HashMap
    map.insert("name", "Deep Mondal");
    map.insert("email", "dm7041979@gmail.com");

    // Retrieve a value associated with a key using `get`
    if let Some(value) = map.get("name") {
        println!("The value associated with 'name' is: {}", value);
    } else {
        println!("Key not found");
    }

    // Retrieve a value that doesn't exist
    if let Some(value) = map.get("age") {
        println!("The value associated with 'age' is: {}", value);
    } else {
        println!("Key not found for 'age'");
    }

}