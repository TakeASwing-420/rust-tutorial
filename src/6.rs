fn main() {
    // Decimal literals (base 10)
    let decimal = 98_222;
    let negative_decimal = -1_234;

    // Hexadecimal literals (base 16)
    let hex = 0xff;
    let hex_with_prefix = 0x2A;

    // Octal literals (base 8)
    let octal = 0o77;
    let octal_with_prefix = 0o12;

    // Binary literals (base 2)
    let binary = 0b1111_0000;
    let binary_with_prefix = 0b1010_1010;

    // Byte (u8) literals
    let byte_literal = b'A'; // Represents the byte value of ASCII 'A' (65)

    println!("Decimal: {}", decimal);
    println!("Negative Decimal: {}", negative_decimal);
    println!("Hexadecimal: {}", hex);
    println!("Hexadecimal with prefix: {}", hex_with_prefix);
    println!("Octal: {}", octal);
    println!("Octal with prefix: {}", octal_with_prefix);
    println!("Binary: {}", binary);
    println!("Binary with prefix: {}", binary_with_prefix);
    println!("Byte literal: {}", byte_literal);
}
