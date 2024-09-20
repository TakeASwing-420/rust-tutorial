use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // Lock stdout for writing

    let byte_array: [u8; 3] = [0x12, 0x34, 0x56];
    let my_array = [3; 4];

    let result: Result<i32, &str> = Ok(10);
    let value = result.unwrap(); // Gets the value (10) or panics if it's an Err


    // Print the byte array
    write!(handle, "Byte array: ").unwrap();
    for b in &byte_array {
        writeln!(handle, "Byte array element in hex: {:02X}", b).unwrap();
    }

    // Print the elements of my_array
    writeln!(handle, "my_array elements:").unwrap();
    for (i, num) in my_array.iter().enumerate() {
        if i > 0 {
            write!(handle, ", ").unwrap(); // Print comma separator
        }
        write!(handle, "{}", num).unwrap(); // Print the number
    }
    writeln!(handle).unwrap(); // Print a newline after the array

    // `handle` goes out of scope here, and the lock is automatically released
}
