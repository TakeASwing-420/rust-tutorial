fn main() {
    let byte: u8 = 0xFF;
    let byte_array: [u8; 3] = [0x12, 0x34, 0x56];

    // Print a single byte
    println!("Byte: {}", byte);
    println!("Byte in hex: {:02X}", byte);

    // Print the byte array
    for b in &byte_array {
        println!("Byte array element in hex: {:02X}", b);
    }

    // Convert a string to bytes and print
    let my_str = "Rust"; //ascii encoding
    let bytes = my_str.as_bytes();
    println!("String as bytes: {:?}", bytes);
}
