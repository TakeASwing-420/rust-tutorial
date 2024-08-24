fn main() {
    let byte: u8 = 0xFF;
    let byte_array: [u8; 3] = [0x12, 0x34, 0x56];
    let my_array = [3; 4];

    // Print a single byte
    println!("Byte: {}", byte);
    println!("Byte in hex: {:02X}", byte);

    // Print the byte array
    for b in byte_array {
        println!("Byte array element in hex: {:02X}", b);
    }

    // Print the elements of my_array
    for (i, num) in my_array.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", num);
    }
    println!(); // To add a newline after the loop

    // Convert a string to bytes and print
    let my_str = "Rust"; // ascii encoding
    let bytes = my_str.as_bytes();
    println!("String as bytes: {:?}", bytes);

}
