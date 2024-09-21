fn main() {
    // 1. Basic formatting
    let name = "Alice";
    let age = 30;
    println!("Hello, my name is {} and I am {} years old.", name, age);

    // 2. Formatting numbers with different bases
    let number = 42;
    println!("Decimal: {}, Hex: {:X}, Binary: {:b}", number, number, number);

    // 3. Floating-point formatting
    let pi = 3.141592653589793;
    println!("Pi: {:.2}, Scientific Notation: {:e}", pi, pi);

    // 4. Right alignment with padding
    let value = 5;
    println!("Right-aligned with spaces: '{:>5}'", value);
    println!("Right-aligned with zeros: '{:0>5}'", value);

    // 5. Left alignment
    let left_aligned = "Rust";
    println!("Left-aligned: '{:0<10}'", left_aligned);

    // 6. Formatting with named arguments
    println!("{name} is {age} years old.", name = name, age = age);

    // 7. Padding with a specific character (using replace method)
    let hex_number = 5;
    let formatted_hex = format!("{:x}", hex_number);
    let padded_hex = if formatted_hex.len() < 2 {
        format!("3{}", formatted_hex)
    } else {
        formatted_hex
    };
    println!("Padded with '3': '{}'", padded_hex);

    // 8. Using a tuple for formatting
    let point = (5, 10);
    println!("Point coordinates: ({}, {})", point.0, point.1);

    // 9. Displaying a list with formatting
    let list = vec![1, 2, 3, 4, 5];
    println!("List: {:?}", list);

    // 10. Formatting with a trait
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Bob"),
        age: 25,
    };
    println!("Person: {:?}", person);
}
