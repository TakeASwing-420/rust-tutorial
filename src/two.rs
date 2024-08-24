pub fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    let sum: i32 = a + b;
    
    let spaces = "   "; // Declare a string with spaces
    #[allow(unused_variables)] // Suppress the unused variable warning
    let mut spaces = spaces.len(); // Redefine `spaces` as its length (3) i.e. shadowing
    println!("Total space count: {spaces}");

    //The following code throws an compile-time error. We are not allowed to change a mutable variables's data type although we can change it's value.

    // let mut spaces = "   ";
    // spaces = spaces.len();
    
    println!("The sum of {} and {} is {}", a, b, sum);
}