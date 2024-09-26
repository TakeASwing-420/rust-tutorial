// A function that takes two string slices and returns the one with the longer lifetime.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("Rust");
    let string2 = "Programming Language";

    // Call `longest` with references to `string1` and `string2`.
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result); // The longest string is: Programming Language

    // Demonstrate lifetime mismatch error.
    // Uncomment the following lines to see a compilation error.
    /*
    let result;
    {
        let temp_string = String::from("Temporary String");
        result = longest(string1.as_str(), temp_string.as_str());
    } // `temp_string` goes out of scope here.
    // println!("The longest string is: {}", result); // Error: `temp_string` does not live long enough.
    */
}
