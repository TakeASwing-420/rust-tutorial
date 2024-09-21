use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new(); // Correct way to create a new String
    print!("Enter input: "); // Print without newline
    io::stdout().flush().expect("Failed to flush stdout"); // Manually flush stdout
    io::stdin().read_line(&mut input).expect("we couldn't read!");

    let (index, substring) = just_for_fun(&input);

    println!("Resulting index: {}", index); // Print the result
    println!("Substring up to index: '{}'", substring); // Print the substring
}

fn just_for_fun(input: &String) -> (usize, String) {
    let mut index = input.len(); // Default to the length of the input if no space is found
    for (i, &byte) in input.as_bytes().iter().enumerate() {
        if byte == b' ' {
            index = i + 1; // Set index to the first space found
            break;
        }
    }
    (index, input[..index].to_string()) // Return both index and substring
}
